use std::error::Error;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Lines;
use std::io::prelude::*;
use std::path::Path;
use std::fs::metadata;
use std::collections::VecDeque;
use std::io::BufReader;
use util;
use db::Database;
use db::models::*;
use std::path::PathBuf;
use std::fs::OpenOptions;

pub struct FileData {
  pub hash: u64,
  pub lines: String
}

impl FileData {
  fn new(file: String, hash: u64) -> FileData {
    FileData {lines: file, hash: hash}
  }
}

pub struct FileIterator {
  explorablePaths: VecDeque<String>,
}

impl FileIterator {
  fn new(path: &String) -> FileIterator {
    let mut queue = VecDeque::<String>::new();
    queue.push_front(path.clone());

    FileIterator { explorablePaths: queue }
  }
}

impl Iterator for FileIterator {
  type Item = FileData;

  fn next(&mut self) -> Option<FileData> {
    if self.explorablePaths.len() == 0 {
      return None;
    }

    let path = self.explorablePaths.pop_front().unwrap();

    let md = metadata(&path).unwrap();

    let isFile = md.is_file();
    let isDir = md.is_dir();

    if should_ignore_path(path.clone()) {
      return self.next();
    }

    if isDir {
      let entries = fs::read_dir(&Path::new(&path)).unwrap();

      for entry in entries {
        match entry {
          Ok(dir) => {
            let dirPath = dir.path().to_str().unwrap().to_string();

            self.explorablePaths.push_back(dirPath);
          },
          Err(E) => {}
        }
      }

      return self.next();
    }

    match load_file(path) {
      Ok(data) => Some(data),
      Err(e) => self.next(),
    }
  }
}

fn should_ignore_path (path: String) -> bool {
  let mut dir = PathBuf::from(path);

  for path in dir.iter() {
    if path.to_str().unwrap() == ".fuzzy" {
      return true;
    }
  }

  return false;
}

fn load_file (path: String) -> Result<FileData, io::Error> {
  let osPath = Path::new(&path);
  let display = osPath.display();

  let mut file = try!(File::open(&osPath));

  let mut s = String::new();
  try!(file.read_to_string(&mut s));

  let hashVal = try!(util::hash_file(path.clone()));

  let file_lines = FileData::new(s, hashVal);

  Ok(file_lines)
}

pub fn read(path: &String) -> FileIterator {
  return FileIterator::new(path);
}

pub fn open_or_create_in_fuzzy(
    path: &Path) ->
      Result<String, Box<Error + Send + Sync>> {
  let mut handle = try!(OpenOptions::new()
          .read(true)
          .create(true)
          .open(path));

  let mut s = String::new();
  try!(handle.read_to_string(&mut s));

  Ok(s)
}

pub fn get_data_files(path: String) -> Result<(), Box<Error + Send + Sync>> {
  let mut dir = PathBuf::from(path);
  dir.push(".fuzzy");

  fs::create_dir(&dir);

  Ok(())
}
