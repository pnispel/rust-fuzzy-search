use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::hash::{Hash, SipHasher, Hasher};

#[derive(Hash)]
struct HashableFile {
  bytes: Vec<u8>
}

pub fn hash<T: Hash>(t: &T) -> u64 {
  let mut s = SipHasher::new();
  t.hash(&mut s);
  s.finish()
}

pub fn hash_file(path: String) -> Result<u64, io::Error>  {
  let mut file = try!(File::open(path));
  let mut s = String::new();

  try!(file.read_to_string(&mut s));

  let mut hashableFile = HashableFile{bytes: s.into_bytes()};

  Ok(hash(&hashableFile))
}
