mod crust_file;
mod crust_packed;
use crust_packed::*;

fn main() {
  let test = match CrustPacked::from_file("test.crust") {
    Ok(d) => {d},
    Err(e) => {eprintln!("Error creating CrustPacked: {}", e); return;}
  };

  for file in &test.files {
    println!("{}: {}, {}", file.filename, file.data_len, file.file_data.len());
  }

  match test.unpack_into("data") {
    Ok(_) => {},
    Err(e) => {eprintln!("Error unpacking CrustPacked: {}", e)}
  }
}
