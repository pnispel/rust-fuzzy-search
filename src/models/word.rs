use std::collections::hash_map::{HashMap, Entry};
use csv::{Reader, Writer};
use super::WORDS;
use super::NEXT_WORD_ID;
use std::error::Error;
use index::file;
use std::path::PathBuf;
use util::string;
use std::sync::MutexGuard;
use std::cmp::Ordering;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Word {
  pub id: i32,
  pub text: String,
  pub frequency: i32,
}

impl Word {
  pub fn read_csv(path: &String) -> Result<(), Box<Error + Send + Sync>> {
    let mut dir = PathBuf::from(path);
    let mut WORD_ID = NEXT_WORD_ID.lock().unwrap();
    dir.push(".fuzzy");
    dir.push("words.csv");

    try!(file::open_or_create_in_fuzzy(dir.as_path()));

    let mut rdr = Reader::from_file(dir.as_path()).unwrap().has_headers(false);

    for record in rdr.decode() {
      let model: Word = record.unwrap();
      let id = model.id;

      if id > *WORD_ID {
        *WORD_ID = id;
      }

      WORDS.lock().unwrap().insert(model.text.clone(), model);
    }

    *WORD_ID += 1;

    Ok(())
  }

  pub fn write_csv(path: &String) -> Result<(), Box<Error + Send + Sync>> {
    let mut dir = PathBuf::from(path);
    dir.push(".fuzzy");
    dir.push("words.csv");

    let mut wtr = Writer::from_file(dir.as_path()).unwrap();

    let g_words = WORDS.lock().unwrap();
    let mut words: Vec<&Word> = g_words.iter().map(|(_, word)| word).collect::<Vec<&Word>>();

    words.sort_by(|a, b| {
      let valA = a.frequency;
      let valB = b.frequency;

      if valA < valB {
        return Ordering::Greater;
      } else if valA > valB {
        return Ordering::Less;
      } else {
        return Ordering::Equal;
      }
    });

    for word in words {
      wtr.encode(word);
    }

    Ok(())
  }

  pub fn new(id: i32, text: String) -> Word {
    Word {
      id: id,
      text: text,
      frequency: 1,
    }
  }

  pub fn with_id(id: i32) -> Word {
    Word {
      id: id,
      text: String::new(),
      frequency: 1,
    }
  }

  pub fn with_text(text: String) -> Word {
    Word {
      id: 0,
      text: text,
      frequency: 1,
    }
  }

  pub fn change_frequency(&mut self, val: i32) {
    self.frequency += val;
  }

  pub fn get_next_id(words: MutexGuard<HashMap<String, Word>>) -> i32 {
    let mut next_id = 1;

    if words.len() != 0 {
      next_id = words.values().last().unwrap().id + 1;
    }

    (next_id)
  }

  // pub fn all(db: &DatabaseConnection) -> SqliteResult<Vec<Word>> {
  //   let mut tx = try!(db.prepare("SELECT * FROM word"));

  //   let mut word_list = Vec::<Word>::new();

  //   try!(tx.query(
  //     &[], &mut |row| {
  //       word_list.push(Word {
  //         id: row.get("id"),
  //         text: row.get("text"),
  //         frequency: row.get("frequency"),
  //       });

  //       Ok(())
  //     }));

  //   Ok(word_list)
  // }

  // pub fn all_hashmap_by_text() -> HashMap<String, Word> {
  //   let mut word_list = HashMap::<String, Word>::new();

  //   for word in WORDS.lock().unwrap().iter() {
  //     word_list.insert(word.text.clone(), Word {
  //       id: word.id,
  //       text: word.text.clone(),
  //       frequency: word.frequency,
  //     });
  //   }

  //   (word_list)
  // }

  // pub fn update_from_tokens(tokens: &Vec<String>) {
  //   let mut withoutStops = string::without_stops(tokens);
  //   let mut g_words = WORDS.lock().unwrap();

  //   'outer: for token in withoutStops {
  //     let clone = token.clone();
  //     let mut found = true;

  //     match g_words.entry(token) {
  //       Entry::Occupied(mut entry) => {
  //         entry.get_mut().change_frequency(1);
  //       }
  //       Entry::Vacant(entry) => {
  //         let new_word = Word::new(Word::get_next_id(g_words), token);

  //         entry.insert(new_word);
  //         break 'outer;
  //       }
  //     }
  //   }
  // }

  // pub fn create(&self, db: &DatabaseConnection) -> SqliteResult<()> {
  //   let mut tx = try!(db.prepare("
  //     INSERT INTO word (text, frequency)
  //     VALUES ($1, $2)"));

  //   let changes = try!(tx.update(&[&self.text, &self.frequency]));

  //   Ok(())
  // }

  // pub fn get(&mut self, db: &DatabaseConnection) -> SqliteResult<Word> {
  //   let mut tx = try!(db.prepare("
  //     SELECT * FROM word WHERE id=$1"));

  //   self.id.to_sql(&mut tx, 1);

  //   let mut word = Word::new();

  //   try!(tx.query(
  //     &[], &mut |row| {
  //       word = Word {
  //         id: row.get("id"),
  //         text: row.get("text"),
  //         frequency: row.get("frequency"),
  //       };

  //       Ok(())
  //     }));

  //   Ok(word)
  // }

  pub fn create(&mut self) {
    let mut words = WORDS.lock().unwrap();

    if self.id == 0 {
      let next_id = words.values().last().unwrap().id + 1;

      self.id = next_id;
    }

    words.insert(self.text.clone(), Word {
      id: self.id,
      text: self.text.clone(),
      frequency: self.frequency
    });
  }

  pub fn update(&mut self) {
    // if self.id == 0 {
    //   self.create();

    //   return;
    // }

    // let mut words = WORDS.lock().unwrap();
    // let mut filtered = words.iter().filter(|x| x.id == self.id);
    // let mut word: Word = filtered;

    // word.id = self.id;
    // word.text = self.text;
    // word.frequency = self.frequency;
  }

  // pub fn delete(&self, db: &DatabaseConnection) -> SqliteResult<()> {
  //   let mut tx = try!(db.prepare("DELETE FROM word WHERE id=$1"));

  //   try!(tx.update(&[&self.id]));

  //   Ok(())
  // }
}
