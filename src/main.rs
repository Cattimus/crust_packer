mod crust_file;
mod crust_packed;
use crust_packed::*;

fn main() {
  let test = CrustPacked::unpack_file("test.crust");
  let test = test.unwrap();

  for file in &test.files {
    println!("{}", file.filename);
  }
}
