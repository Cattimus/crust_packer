use std::fs;
use std::path::Path;
use std::io::Write;

use crate::crust_file::*;

#[allow(dead_code)]
pub fn get_filenames(path: &str) -> Vec<String>{
  //get a list of all files from the directory
  let mut dirs: Vec<String> = Vec::new();
  dirs.push(path.to_string());
  let mut files: Vec<String> = Vec::new();

  //iterate over directory
  while dirs.len() > 0 {
    let current_dir = dirs.pop().unwrap();

    let dir = fs::read_dir(&current_dir);
    if dir.is_err() {
      eprintln!("Error reading from directory: {}", &current_dir);
      continue;
    }

    let dir = dir.unwrap();
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
      let crust_file = CrustFile::from(&file);
      if crust_file.is_some() {
        files.push(crust_file.unwrap());
      }
    }

    return Some (
      CrustPacked {
        file_count: files.len() as u32,
        files: files
      })
  }

  //unpack a crust file into an object
  pub fn unpack_file(path: &str) -> Option<Self> {
    //check if file exists and isn't a directory
    let mut file_descriptor = Path::new(path);
    if !file_descriptor.exists() || !file_descriptor.is_file() {
      return None;
    }

    //read file into memory
    let mut file = fs::read(file_descriptor);
    if file.is_err() {
      eprintln!("Error opening file: {}", path);
      return None;
    }

    //check that crust header exists in file
    let mut file = file.unwrap();
    let header = "CRuST";
    let slice =& file[0..5];

    if !header.as_bytes().eq(slice) {
      eprintln!("Crust header not found in file: {}", path);
      return None;
    }

    //create crust_file objects from bytes
    let file_count = u32::from_le_bytes(file[5..9].try_into().unwrap());
    println!("file count: {}", file_count);
    let mut files: Vec<CrustFile> = Vec::new();

    //iterate through files
    let mut i: usize = 9;
    for x in 0..file_count {
      let obj = CrustFile::from_bytes(&file[i..]).unwrap();
      i += 7 as usize + obj.extension_len as usize + obj.name_len as usize + obj.data_len as usize;
      files.push(obj);
    }

    return Some(
      CrustPacked {
        file_count,
        files
      }
    )
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
    let file = fs::File::create(&filename);
    if file.is_err() {
      eprintln!("Error creating file: {}", &filename);
      return;
    }

    let error_msg = "Error writing to file: ".to_string() + filename;

    let mut file = file.unwrap();
    let data = self.as_bytes();
    file.write_all(&&data).expect(&error_msg);
  }
}