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
    let meta = match fs::metadata(filename) {
      Ok(d) => {d},
      Err(_) => {return None}
    };

    //file is actually a directory
    if !meta.is_file() {
      return None;
    }

    let data = match fs::read(filename) {
      Ok(d) => {d},
      Err(_) => {eprintln!("CrustFile.read(): Error reading file {}", filename); return None}
    };

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

  //Create a crust file from a slice of bytes
  pub fn from_bytes(data: &[u8]) -> Result<Self, &str> {
    //data must at least contain the header objects
    if data.len() < 7 {
      return Err("Buffer does not contain enough data for basic header.");
    }

    let mut i: usize = 0;

    let extension_len = data[i];
    i += 1;

    let name_len = u16::from_le_bytes(data[i..i+2].try_into().unwrap());
    i += 2;

    let data_len = u32::from_le_bytes(data[i..i+4].try_into().unwrap());
    i += 4;

    //if there isn't enough data left in the buffer to read
    if data.len() < 7 + name_len as usize + extension_len as usize + data_len as usize {
      return Err("Header may be corrupted or file may be invalid. Not enough bytes.");
    }

    let filename = match std::str::from_utf8(&data[i..i+name_len as usize]) {
      Ok(str) => {str.to_string()},
      Err(_) => {return Err("Failed to convert filename into valid utf-8 string.")}
    };
    i += name_len as usize;

    let extension = match std::str::from_utf8(&data[i..i+extension_len as usize]) {
      Ok(str) => {str.to_string()},
      Err(_) => {return Err("Failed to convert extension into valid utf-8 string.")}
    };
    i += extension_len as usize;

    let file_data = data[i..i+data_len as usize].to_vec();

    return Ok(
      CrustFile {
        extension_len,
        name_len,
        data_len,
        filename,
        extension,
        file_data
      }
    );
  }

  pub fn extract_to(&self, path: &str) -> Result<i32, &str>{
    //create all necessary directories to write the file
    if fs::create_dir_all(path).is_err() {
      return Err(&format!("Could not create directory {}", path));
    }

    //create our new file path
    let filename = path.to_string() + "/" + &self.filename;

    //create file
    let mut file = match fs::File::create(&filename) {
      Ok(d) => {d},
      Err(_) => {return Err(&format!("Could not write to file {}", filename))}
    };

    //write file
    if file.write_all(self.file_data.as_slice()).is_err() {
      return Err(&format!("Could not write file {}", filename));
    }

    //need this for result
    return Ok(0);
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