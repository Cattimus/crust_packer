use std::fs;
use std::path::Path;
use std::io::Write;

use crate::crust_file::*;

#[allow(dead_code)]
fn get_filenames(path: &str) -> Vec<String>{
  //get a list of all files from the directory
  let mut dirs: Vec<String> = Vec::new();
  dirs.push(path.to_string());
  let mut files: Vec<String> = Vec::new();

  //iterate over directory
  while dirs.len() > 0 {
    //This is fine and should never panic, since we check dirs.len() > 0 before this. keyword should.
    let current_dir = dirs.pop().unwrap();

    let dir = match fs::read_dir(&current_dir) {
      Ok(d) => {d},
      Err(_) => {eprintln!("CrustPacked.from_dir(): Error reading from directory {}", &current_dir); continue}
    };

    for file in dir {
      if file.is_err() {
        continue;
      }
      let file = file.unwrap();
      let metadata = file.metadata();
      if metadata.is_err() {
        continue;
      }

      //push the data to the correct buffer
      let metadata = metadata.unwrap();
      let filename = file.path().to_str().unwrap().to_string();
      if metadata.is_dir() {
        dirs.push(filename);
      } else {
        files.push(filename);
      }
    }
  }

  return files;
}

pub struct CrustPacked {
  pub file_count: u32,
  pub files: Vec<CrustFile>
}

#[allow(dead_code)]
impl CrustPacked {
  pub fn from_dir(path: &str) -> Option<Self> {
    //directory does not exist
    if !Path::new(path).exists() {
      return None
    }

    let filenames = get_filenames(path);
    let mut files: Vec<CrustFile> = Vec::new();

    //create objects for all the files
    for file in filenames {
      match CrustFile::from(&file) {
        Some(d) => {files.push(d)},
        None => {}
      };
    }

    return Some (
      CrustPacked {
        file_count: files.len() as u32,
        files: files
      })
  }

  //unpack a crust file into an object
  pub fn from_file(path: &str) -> Option<Self> {
    //check if file exists and isn't a directory
    let file_descriptor = Path::new(path);
    if !file_descriptor.exists() || !file_descriptor.is_file() {
      return None;
    }

    //read file into memory
    let file = match fs::read(file_descriptor) {
      Ok(d) => {d},
      Err(_) => {eprintln!("CrustPacked.from_file(): Error opening file {}", path); return None}
    };

    //check that crust header exists in file
    let header = "CRuST";
    let slice =& file[0..5];

    if !header.as_bytes().eq(slice) {
      eprintln!("CrustPacked.from_file(): Crust header not found in file {}", path);
      return None;
    }

    //create crust_file objects from bytes
    let file_count = u32::from_le_bytes(file[5..9].try_into().unwrap());
    let mut files: Vec<CrustFile> = Vec::new();

    //iterate through files
    let mut i: usize = 9;
    for _ in 0..file_count {
      match CrustFile::from_bytes(&file[i..]) {
        Some(obj) => {
          i += 7 as usize + obj.extension_len as usize + obj.name_len as usize + obj.data_len as usize;
          files.push(obj);
        },

        None => {
          eprintln!("CrustPacked.from_file(): Failed to parse file. File may be invalid or corrupted.");
          return None;
        }
      }
    }

    return Some(
      CrustPacked {
        file_count,
        files
      }
    )
  }

  //unpack files in specified directory
  pub fn unpack_into(&self, path: &str) {
    //check if path already exists
    let desired_path = Path::new(path);
    if desired_path.exists() && !desired_path.is_dir() {
      eprintln!("CrustPacked.unpack_into(): An object already exists at {} that is not a directory.", path);
      return;
    }

    //attempt to create directory if it doesn't exist
    if !desired_path.exists() {
      if fs::create_dir(desired_path).is_err() {
        eprintln!("CrustPacked.unpack_into(): Could not create directory {}", path);
        return;
      }
    }

    //write files into directory
    for i in 0..self.file_count {
      let file = &self.files[i as usize];
      let file_path = desired_path.join(&file.filename);
      if fs::write(&file_path, &file.file_data.as_slice()).is_err() {
        eprintln!("CrustPacked.unpack_into(): Error writing file {}", &file_path.to_str().unwrap());
      }
    }
  }

  //return new copy of self as a vec of u8
  pub fn as_bytes(&self) -> Vec<u8>{
    let mut buf: Vec<u8> = Vec::new();
    
    let mut identifier = "CRuST".as_bytes().to_vec();
    buf.append(&mut identifier);

    let mut file_count = self.file_count.to_le_bytes().to_vec();
    buf.append(&mut file_count);

    //append files to the byte array
    for file in &self.files {
      let mut temp: Vec<u8> = file.as_bytes();
      buf.append(&mut temp);
    }

    return buf;
  }

  pub fn write(&self, filename: &str) {
    let mut file = match fs::File::create(&filename) {
      Ok(d) => {d},
      Err(_) => {eprintln!("CrustPacked.write(): Error creating file {}", filename); return}
    };

    let data = self.as_bytes();
    match file.write_all(&&data) {
      Ok(_) => {},
      Err(_) => {eprintln!("CrustPacked.write(): Error writing to file {}", filename); return}
    };
  }
}