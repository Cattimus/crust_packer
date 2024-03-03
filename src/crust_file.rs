use std::fs;
use std::io::Write;

pub struct CrustFile {
  pub extension_len: u8,
  pub name_len: u16,
  pub data_len: u32,
  pub filename: String,
  pub extension: String,
  pub file_data: Vec<u8>
}

#[allow(dead_code)]
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
    let file = fs::File::create(&filename);
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