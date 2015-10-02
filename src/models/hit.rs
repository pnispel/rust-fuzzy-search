use std::collections::hash_map::{HashMap, Entry};
use csv::{Reader, Writer};
use super::{HITS, WORDS, NEXT_WORD_ID};
use std::error::Error;
use index::file;
use std::path::PathBuf;
use util::string;
use std::sync::MutexGuard;
use models::Word;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Hit {
  position: i32,
  word_id: i32,
  document_id: i32,
}

impl Hit {
  pub fn read_csv(path: &String) -> Result<(), Box<Error + Send + Sync>> {
    let mut dir = PathBuf::from(path);
    dir.push(".fuzzy");
    dir.push("hits.csv");

    try!(file::open_or_create_in_fuzzy(dir.as_path()));

    let mut rdr = Reader::from_file(dir.as_path()).unwrap().has_headers(false);

    for record in rdr.decode() {
      let record: Hit = record.unwrap();

      HITS.lock().unwrap().push(record);
    }

    Ok(())
  }

  pub fn write_csv(path: &String) -> Result<(), Box<Error + Send + Sync>> {
    let mut dir = PathBuf::from(path);
    dir.push(".fuzzy");
    dir.push("hits.csv");

    let mut wtr = Writer::from_file(dir.as_path()).unwrap();

    for record in HITS.lock().unwrap().iter() {
      wtr.encode(record);
    }

    Ok(())
  }

  // pub fn delete(&self, db: &DatabaseConnection) -> SqliteResult<()> {
  //   let mut tx = try!(db.prepare(
  //     "DELETE FROM hit WHERE word_id=$1 document_id=$2 line=$3"));

  //   try!(tx.update(&[&self.word_id]));
  //   try!(tx.update(&[&self.document_id]));
  //   try!(tx.update(&[&self.line]));

  //   Ok(())
  // }

  // pub fn word(&self, db: &DatabaseConnection) -> SqliteResult<Word> {
  //   let mut w = Word::with_id(self.word_id);

  //   let ret = try!(w.get(db));

  //   Ok(ret)
  // }

  // pub fn create(&self, db: &DatabaseConnection) -> SqliteResult<()> {
  //   let mut tx = try!(db.prepare("
  //     INSERT INTO hit (line, word_id, document_id)
  //     VALUES ($1, $2, $3)"));

  //   let changes = try!(tx.update(&[
  //     &self.line, &self.word_id, &self.document_id
  //   ]));

  //   Ok(())
  // }

  pub fn add_from_tokens(tokens: &Vec<String>, doc_id: i32, line_id: i32) {
    let mut withoutStops = string::without_stops(tokens);
    let mut g_words = WORDS.lock().unwrap();
    let mut g_hits = HITS.lock().unwrap();
    let mut position = 0;

    for token in withoutStops {
      let clone = token.clone();
      let mut found = false;
      let mut word_id = 0;
      position += 1;

      match g_words.entry(token) {
        Entry::Occupied(mut entry) => {
          let entry = entry.get_mut();
          word_id = entry.id;

          entry.change_frequency(1);
        }
        Entry::Vacant(entry) => {
          let mut next_id = NEXT_WORD_ID.lock().unwrap();
          word_id = *next_id;
          *next_id += 1;

          let new_word = Word::new(word_id, clone);

          entry.insert(new_word);
        }
      }

      g_hits.push(Hit {
        position: position,
        word_id: word_id,
        document_id: doc_id,
      });
    }
  }

  //   let mut words = try!(Word::all_hashmap_by_text(db));
  //   let mut hits = Vec::<Hit>::new();
  //   let mut i = 0;

  //   for token in tokens {
  //     let clone = token.clone();

  //     match words.entry(clone) {
  //       Entry::Occupied(mut entry) => {
  //         let word_id = entry.get_mut().id;

  //         hits.push(Hit {
  //           line: line_id,
  //           word_id: word_id,
  //           document_id: doc_id,
  //         });
  //       }
  //       Entry::Vacant(entry) => {
  //         // shouldnt happen since we just created all of these
  //       }
  //     };

  //     i += 1;
  //   }

  //   for hit in hits {
  //     hit.create(db);
  //   }

  //   Ok(())
  // }
}
