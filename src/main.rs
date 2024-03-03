use std::fs::{self, Metadata};
use std::io::Write;
use std::path::Path;

struct CrustFile {
  pub extension_len: u8,
  pub name_len: u16,
  pub data_len: u32,
  pub filename: String,
  pub extension: String,
  pub file_data: Vec<u8>
}

impl CrustFile{
  pub fn from(filename: &str) -> Option<CrustFile> {
    //get metadata for file
    let meta = fs::metadata(filename);
    if meta.is_err() {
      return None;
    }

    //file is actually a directory
    let meta = meta.unwrap();
    if !meta.is_file() {
      return None;
    }

    let data = fs::read(filename);
    if data.is_err() {
      eprintln!("Error reading file: {}", filename);
      return None;
    }
    let data = data.unwrap();

    //get the file name from the file path provided
    let filename = filename.replace("\\", "/");
    let splits = filename.split("/").collect::<Vec<&str>>();
    let filename = splits[splits.len() - 1].to_string();

    //holy mother of god what is this syntax
    let extension = filename.split(".").collect::<Vec<&str>>()[1].to_string();

    //now we have our file object
    return Some(CrustFile {
      extension_len: extension.len() as u8,
      name_len: filename.len() as u16,
      data_len: data.len() as u32,
      filename: filename.to_string(),
      extension: extension,
      file_data: data
    });
  }

  pub fn extract_to(&self, path: &str) {
    //create all necessary directories to write the file
    if fs::create_dir_all(path).is_err() {
      eprintln!("Error creating directory {}", path);
      return;
    }

    //create our new file path
    let filename = path.to_string() + "/" + &self.filename;

    //create file
    let mut file = fs::File::create(&filename);
    if file.is_err() {
      eprintln!("Error writing file {}", filename);
      return;
    }

    //write file
    let mut file = file.unwrap();
    if file.write_all(self.file_data.as_slice()).is_err() {
      eprintln!("Error writing file {}", filename);
    }
  }

  //return a new copy of self as a vec of u8
  pub fn as_bytes(&self) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::new();
    buf.push(self.extension_len);

    let mut name_buf = self.name_len.to_le_bytes().to_vec();
    buf.append(&mut name_buf);

    let mut data_len = self.data_len.to_le_bytes().to_vec();
    buf.append(&mut data_len);

    let mut filename = self.filename.as_bytes().to_vec();
    buf.append(&mut filename);

    let mut extension = self.extension.as_bytes().to_vec();
    buf.append(&mut extension);

    let mut file_data = self.file_data.clone();
    buf.append(&mut file_data);

    return buf;
  }
}

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

struct CrustPacked {
  pub file_count: u32,
  pub files: Vec<CrustFile>
}

impl CrustPacked {
  pub fn from_dir(path: &str) -> Self {
    let filenames = get_filenames(path);
    let mut files: Vec<CrustFile> = Vec::new();

    //create objects for all the files
    for file in filenames {
      let crust_file = CrustFile::from(&file);
      if crust_file.is_some() {
        files.push(crust_file.unwrap());
      }
    }

    CrustPacked {
      file_count: files.len() as u32,
      files: files
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

fn main() {
  let test = CrustPacked::from_dir("test");

  for file in &test.files {
    println!("{}", file.filename);
  }

  test.write("test.crust")
}
