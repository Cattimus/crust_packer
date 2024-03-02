struct CrustFile {
  extension_len: u8,
  name_len: u16,
  data_len: u32,
  filename: String,
  extension: String,
  file_data: Vec<u8>
}

struct CrustPacked {
  identifier: String,
  file_count: u32,
  files: Vec<CrustFile>
}

fn main() {
  println!("Hello, world!");
}
