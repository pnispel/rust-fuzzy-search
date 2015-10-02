pub use self::cluster::Cluster;
pub use self::word::Word;
pub use self::document::Document;
pub use self::hit::Hit;
use std::collections::hash_map::{HashMap, Entry};
// pub use self::cluster_link::ClusterLink;

mod cluster;
mod word;
mod document;
mod hit;
// mod cluster_link;

use std::sync::Mutex;
use std::error::Error;

lazy_static! {
  static ref DOCUMENTS: Mutex<Vec<Document>> = Mutex::new(vec![]);
  static ref WORDS: Mutex<HashMap<String, Word>> = Mutex::new(HashMap::<String, Word>::new());
  static ref HITS: Mutex<Vec<Hit>> = Mutex::new(vec![]);
  static ref CLUSTERS: Mutex<Vec<Cluster>> = Mutex::new(vec![]);
  // static ref CLUSTER_LINKS: Mutex<Vec<ClusterLink>> = Mutex::new(vec![]);

  static ref NEXT_WORD_ID: Mutex<i32> = Mutex::new(0);
}

pub fn init(path: &String) -> Result<(), Box<Error + Send + Sync>> {
  try!(Document::read_csv(path));
  try!(Word::read_csv(path));
  try!(Hit::read_csv(path));

  Ok(())
}

pub fn finish(path: &String) -> Result<(), Box<Error + Send + Sync>> {
  Cluster::run();

  try!(Document::write_csv(path));
  try!(Word::write_csv(path));
  try!(Hit::write_csv(path));

  Ok(())
}
