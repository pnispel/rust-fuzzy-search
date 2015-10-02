use std::collections::hash_map::{HashMap};
use csv::{Reader, Writer};
use super::DOCUMENTS;
use std::error::Error;
use index::file;
use std::path::PathBuf;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Document {
  pub id: i32,
  pub hash: u64,
}

impl Document {
  pub fn read_csv(path: &String) -> Result<(), Box<Error + Send + Sync>> {
    let mut dir = PathBuf::from(path);
    dir.push(".fuzzy");
    dir.push("documents.csv");

    try!(file::open_or_create_in_fuzzy(dir.as_path()));

    let mut rdr = Reader::from_file(dir.as_path()).unwrap().has_headers(false);

    for record in rdr.decode() {
      let record: Document = record.unwrap();

      println!("{}", record.id);

      DOCUMENTS.lock().unwrap().push(record);
    }

    Ok(())
  }

  pub fn write_csv(path: &String) -> Result<(), Box<Error + Send + Sync>> {
    let mut dir = PathBuf::from(path);
    dir.push(".fuzzy");
    dir.push("documents.csv");

    let mut wtr = Writer::from_file(dir.as_path()).unwrap();

    for record in DOCUMENTS.lock().unwrap().iter() {
      wtr.encode(record);

      println!("{}", record.id);
    }

    Ok(())
  }
  // pub fn update_by_hash(&mut self, db: &DatabaseConnection) -> SqliteResult<()> {
  //   let mut tx = try!(db.prepare("SELECT id FROM document WHERE hash=$1"));

  //   self.hash.to_sql(&mut tx, 1);

  //   try!(tx.query(
  //     &[], &mut |row| {
  //       self.id = row.get("id");

  //       Ok(())
  //     }));

  //   Ok(())
  // }

  pub fn create(&mut self) {
    let mut docs = DOCUMENTS.lock().unwrap();

    if self.id == 0 {
      let mut next_id = 1;

      if docs.len() != 0 {
        next_id = docs.last().unwrap().id + 1;
      }

      self.id = next_id;
    }

    docs.push(Document {
      id: self.id,
      hash: self.hash
    });
  }

  // pub fn remove_all(db: &DatabaseConnection) -> SqliteResult<()> {
  //   let mut tx = try!(db.prepare("DELETE FROM document"));

  //   let changes = try!(tx.update(&[]));

  //   Ok(())
  // }

  pub fn all_hashmap() -> HashMap <u64, i32> {
    let mut doc_list = HashMap::<u64, i32>::new();

    for doc in DOCUMENTS.lock().unwrap().iter() {
      doc_list.insert(doc.hash, doc.id);
    }

    (doc_list)
  }

  // pub fn all(db: &DatabaseConnection) -> SqliteResult<Vec<Document>> {
  //   let mut tx = try!(db.prepare("SELECT * FROM document"));

  //   let mut doc_list = Vec::<Document>::new();

  //   try!(tx.query(
  //     &[], &mut |row| {
  //       // TODO: this could grow too large for stack?
  //       doc_list.push(Document {
  //         id: row.get("id"),
  //         hash: row.get("hash"),
  //       });

  //       Ok(())
  //     }));

  //   Ok(doc_list)
  // }

  // pub fn hits(&self, db: &DatabaseConnection) -> SqliteResult<Vec<Hit>> {
  //   let mut tx = try!(db.prepare("SELECT * FROM hit WHERE document_id=$1"));

  //   self.id.to_sql(&mut tx, 1);

  //   let mut hit_list = Vec::<Hit>::new();

  //   try!(tx.query(
  //     &[], &mut |row| {
  //       hit_list.push(Hit {
  //         line: row.get("line"),
  //         word_id: row.get("word_id"),
  //         document_id: row.get("document_id"),
  //       });

  //       Ok(())
  //     }));

  //   Ok(hit_list)
  // }

  // fn default() -> Document {
  //   Document { id: 0, hash: 0 }
  // }

  // pub fn new(hash: u64, id: i32) -> Document {
  //   Document {
  //     id: id,
  //     hash: hash
  //   }
  // }

  // pub fn with_id(id: i32) -> Document {
  //   Document {
  //     id: id,
  //     hash: 0
  //   }
  // }
}
