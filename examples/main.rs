extern crate fuzzy;

use fuzzy::index;

use std::error::Error;
use std::io::prelude::*;
use std::path::Path;


fn main() {
  match index::run(String::from("./examples/text/3.txt")) {
    Ok(r) => r,
    Err(e) => panic!(e)
  };
}
