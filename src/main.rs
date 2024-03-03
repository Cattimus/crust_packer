mod crust_file;
mod crust_packed;
use crust_packed::*;

fn main() {
  let test = CrustPacked::from_dir("test");

  for file in &test.files {
    println!("{}", file.filename);
  }

  test.write("test.crust")
}
