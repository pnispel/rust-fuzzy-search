use sqlite3::{
  DatabaseConnection,
  StatementUpdate,
  SqliteResult,
  ResultRowAccess,
  Query,
  PreparedStatement
};

use sqlite3::types::{FromSql, ToSql};

use std::collections::hash_map::{HashMap, Entry};
use std::collections::HashSet;
use std::error::Error;

use util::string;
use util::STOP_WORDS;






