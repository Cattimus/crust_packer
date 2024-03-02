use std::fs::{self, Metadata};
use std::io::Write;

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
    let filename = path.to_string() + "/" + &self.filename;

    let mut file = fs::File::create(&filename);
    if file.is_err() {
      eprintln!("Error writing file {}", filename);
      return;
    }
    let mut file = file.unwrap();
    if file.write_all(self.file_data.as_slice()).is_err() {
      eprintln!("Error writing file {}", filename);
    }
  }
}

struct CrustPacked {
  identifier: String,
  file_count: u32,
  files: Vec<CrustFile>
}

fn main() {
  let test = CrustFile::from("src/main.rs");
  if test.is_some() {
    let test = test.unwrap();
    println!("filename: {}", test.filename);
    println!("file data size: {}", test.data_len);
    println!("extension: {}", test.extension);

    test.extract_to("test");
  }
}
