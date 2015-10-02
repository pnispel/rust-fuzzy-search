#![feature(ascii, btree_range, collections_bound, clone_from_slice, drain)]
#![crate_name = "fuzzy"]
#![crate_type = "lib"]

extern crate regex;
extern crate sqlite3;
extern crate time;
extern crate csv;
extern crate rustc_serialize;

#[macro_use]
extern crate automata;

#[macro_use]
extern crate lazy_static;

pub mod util;
pub mod index;
pub mod db;
pub mod models;

