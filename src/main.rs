mod crust_file;
mod crust_packed;
use crust_packed::*;

fn main() {
  let test = CrustPacked::from_file("test.crust");
  let test = test.unwrap();

  for file in &test.files {
    println!("{}: {}, {}", file.filename, file.data_len, file.file_data.len());
  }

  test.unpack_into("data");
}
