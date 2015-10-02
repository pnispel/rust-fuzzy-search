use time::Timespec;
use time;

use sqlite3::access::ByFilename;
use sqlite3::access::flags::OpenFlags;
use std::default::Default;

use db::models::*;

use std::error::Error;
use std::path::PathBuf;

use sqlite3::{
  DatabaseConnection,
  Query,
  ResultRowAccess,
  SqliteResult,
  StatementUpdate,
};

pub struct Database {
  pub conn: DatabaseConnection
}

impl Database {
  pub fn new(path: String) -> Result<Database, Box<Error + Send + Sync>> {
    let mut dir = PathBuf::from(path);
    dir.push(".fuzzy");
    dir.push("test.sqlite");

    let filename = dir.to_str().unwrap();

    let mut byFilename = ByFilename {
      filename: filename,
      flags: Default::default(),
    };

    let mut conn = try!(DatabaseConnection::new(byFilename));

    let mut db = Database {
      conn: conn,
    };

    db.build();

    Ok(db)
  }

  pub fn build(&mut self) -> SqliteResult<()> {
    self.conn.exec("CREATE TABLE IF NOT EXISTS word (
      id INTEGER PRIMARY KEY,
      text VARCHAR NOT NULL UNIQUE,
      frequency INTEGER
      )");

    self.conn.exec("CREATE TABLE IF NOT EXISTS hit (
      line INTEGER,
      word_id INTEGER,
      document_id INTEGER
      )");

    self.conn.exec("CREATE TABLE IF NOT EXISTS document (
      id INTEGER PRIMARY KEY,
      hash INTEGER UNIQUE
      )");

    self.conn.exec("CREATE TABLE IF NOT EXISTS cluster_link (
      id INTEGER PRIMARY KEY,
      word_id INTEGER,
      cluster_id INTEGER
      )");

    self.conn.exec("CREATE TABLE IF NOT EXISTS cluster (
      id INTEGER PRIMARY KEY,
      size INTEGER
      )")
  }
}



// pub fn io() -> SqliteResult<Vec<Person>> {
//   let file = ByFilename {filename: "test.sqlite", flags: Default::default()};
//   let mut conn = try!(DatabaseConnection::new(file));

//   let me = Person {
//     id: 0,
//     name: format!("Dan"),
//     time_created: time::get_time(),
//   };

//   {
//     let mut tx = try!(conn.prepare("INSERT INTO person (name, time_created)
//                VALUES ($1, $2)"));
//     let changes = try!(tx.update(&[&me.name, &me.time_created]));
//     assert_eq!(changes, 1);
//   }

//   // let mut stmt = try!(conn.prepare("SELECT id, name, time_created FROM person"));

//   // let mut ppl = vec!();
//   // try!(stmt.query(
//   //   &[], &mut |row| {
//   //     ppl.push(Person {
//   //       id: row.get("id"),
//   //       name: row.get("name"),
//   //       time_created: row.get(2)
//   //     });
//   //

//   //   }));
//   // Ok(ppl)
// }
