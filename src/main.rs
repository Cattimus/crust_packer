mod crust_file;
mod crust_packed;
use crust_packed::*;

fn main() {
  let test = CrustPacked::unpack_file("test.crust");

  if test.is_some() {
    println!("It worked!");
  }
}
