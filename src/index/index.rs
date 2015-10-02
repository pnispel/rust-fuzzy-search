use index::clustering::compute_cluster;
use index::file;
use index::frequency::sort_by_frequency;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::fs::File;
use std::fs;
use util::string;
use db::Database;
use std::error::Error;
use models;
use models::{Document, Word, Hit, Cluster};
use std::path::Path;
use time;
use regex::Regex;
use std::ascii::AsciiExt;
use std::str;
use std::char;

use automata::nfa::Transition;
use automata::nfa::Transition::{Input, Epsilon, Anything};
use automata::{Automaton, DFA, NFA};

use std::collections::{HashSet, HashMap, VecDeque, BTreeSet};

use std::collections::Bound::{Included, Unbounded};

struct State {
  db: Database,
}

fn next_valid(dfa: &DFA<usize, char>, s: Vec<char>) -> Option<Vec<char>> {
  let mut string = s.clone();
  let mut state = dfa.start;
  let mut stack = VecDeque::new();
  let mut broken = false;
  let mut final_i = 0;

  for (i, c) in s.iter().enumerate() {
    let mut clone = s.clone();
    let mut temp: Vec<_> = clone.drain(..i).collect();
    stack.push_front((temp, state, Some(*c)));

    state = match dfa.transitions.get(&(state, Input(*c))) {
      Some(s) => *s,
      None => {
        match dfa.transitions.get(&(state, Anything)) {
          Some(s) => *s,
          None => {
            broken = true;

            break;
          }
        }
      }
    };

    final_i = i;
  }

  if !broken {
    let mut clone = s.clone();
    let mut temp: Vec<_> = clone.drain(..(final_i + 1)).collect();
    stack.push_front((temp, state, None));
  }

  if dfa.accept_states.contains(&state) && !broken {
    return Some(s);
  }

  while stack.len() != 0 {
    let (mut path, mut state, x) = stack.pop_front().unwrap();

    if let Some(c) = next_edge(&dfa, state, x) {
      if let Input(ch) = c {
        path.push(ch);
      }

      state = match dfa.transitions.get(&(state, c)) {
        Some(s) => *s,
        None => {
          match dfa.transitions.get(&(state, Anything)) {
            Some(s) => *s,
            None => {return None}
          }
        }
      };

      if dfa.accept_states.contains(&state) {
        return Some(path);
      }

      stack.push_front((path, state, None));
    }
  }

  None
}

fn next_edge(dfa: &DFA<usize, char>, s: usize, x: Option<char>) -> Option<Transition<char>> {
  let y = match x {
    Some(y) => Input(char::from_u32((y as u32) + 1).unwrap()),
    None => Input('\0'),
  };

  let mut alphabet = HashSet::new();
  for (trans, _) in dfa.transitions.iter() {
      match trans.1 {
          Input(c) => alphabet.insert(Input(c)),
          Anything => alphabet.insert(Anything),
          _ => {false},
      };
  }

  let mut transitions = BTreeSet::new();

  for a in alphabet.iter() {
    if let Some(_) = dfa.transitions.get(&(s, *a)) {
      if y == *a || Anything == *a {
        return Some(y);
      }

      if let Input(trans) = *a {
        transitions.insert(trans);
      }
    }
  }

  if let Input(ch) = y {
    let mut first = transitions.range(Included(&ch), Unbounded::<&char>);

    if let Some(f) = first.nth(0) {
      return Some(Input(*f));
    }
  }

  None
}

fn levenshtein_automata(term: &str, k: usize) -> DFA {
  let mut transitions = map!();

  for (i, c) in term.bytes().enumerate() {
    let j = i as usize;

    for e in 0..(k + 1) {
      transitions.insert(((j, e), Input(c as char)), set!((j + 1, e)));

      if e < k {
        transitions.insert(((j, e), Anything), set!((j, e + 1), (j + 1, e + 1)));
        transitions.insert(((j, e), Epsilon), set!((j + 1, e + 1)));
      }
    }
  }

  let mut final_states = set!();

  let len = term.len() as usize;

  for e in 0..(k + 1) {
    if e < k {
      transitions.insert(((len, e), Anything), set!((len, e + 1)));
    }

    final_states.insert((len, e));
  }

  NFA::new((0,0), final_states, transitions).into_dfa()
}

fn find_all_matches(words: Vec<&str>, word: &str, k: usize) {
  let lev = levenshtein_automata(word, k);

  let mut matches = vec!();
  let mut m = next_valid(&lev, vec!('\0'));

  if None == m {
    return;
  }

  loop {
    if let Some(char_arr) = m {
      let mut string = char_arr.iter().cloned().collect::<String>();
      let s = &string[..];

      let mut next = match words.binary_search(&s) {
        Ok(i) => {
          Some(words[i])
        },
        Err(i) => {
          if i >= words.len() {
            break;
          } else {
            Some(words[i])
          }
        },
      };

      let mut ret = next.unwrap().clone().to_string();

      if ret == s {
        matches.push(ret.clone());
        ret = ret + "\0";
      }

      let s : Vec<_> = ret.chars().collect();

      m = next_valid(&lev, s);
    } else {
      break;
    }
  }
}

pub fn run(path: String) -> Result<(), Box<Error + Send + Sync>> {
  let start = time::now();
  // let splitRegex = Regex::new(r"[^a-zA-Z0-9’']+").unwrap();
  // let display = osPath.display();

  let metadata = try!(fs::metadata(&path));

  let mut file = try!(File::open(&path));
  let mut buf = vec![0u8; metadata.len() as usize];

  try!(file.read(&mut buf));

  buf.make_ascii_lowercase();

  let mut str: &str = try!(str::from_utf8(&buf));
  let mut splits: Vec<&str> = str.split_whitespace().collect();

  splits.sort();

  let mut num_empty = 0;

  for s in &splits {
    if s.len() == 0 {
      num_empty += 1;
    } else {
      break;
    }
  }

  let mut without_empty = splits.split_off(num_empty);

  let mut counts = vec![1];
  let mut counts_index = 0;
  let mut last_split = *without_empty.first().unwrap();

  for split in splits {
    if last_split != split {
      counts_index += 1;
      counts.push(0);
    } else {
      counts[counts_index] += 1;
    }

    last_split = split;
  }

  find_all_matches(without_empty, "test", 2);

  let diff = time::now() - start;

  let ns = diff.num_nanoseconds().unwrap() as f64;
  let ticks = 1000000000.0 as f64;

  println!("time: {}", ns / ticks);

  // let mut splits: Vec<String> = splitRegex.split(&s)
  //   .map(|x| x.to_ascii_lowercase())
  //   .filter(|x| x.len() != 0)
  //   .collect();

  // splits.sort();

  // let diff = start - time::now();
  // let ns = diff.num_nanoseconds().unwrap() as f64;
  // let ticks = 1000000000.0 as f64;
  // println!("{}, {}", ns, ns / ticks);

  // println!("start");
  // try!(file::get_data_files(path.clone()));
  // models::init(&path);

  // let mut docs_hashmap = Document::all_hashmap();
  // let mut files = file::read(&path);

  // for file in files {
  //   let mut buffer = String::new();
  //   let mut line_id = 0;
  //   let mut with_id = false;
  //   let mut ids_to_remove = Vec::<i32>::new();
  //   let mut doc = Document {
  //     hash: file.hash,
  //     id: 0
  //   };

  //   match docs_hashmap.get(&file.hash) {
  //     Some(id) => {
  //       doc.id = *id;
  //     },
  //     None => ()
  //   };

  //   if doc.id != 0 {
  //     with_id = true;
  //     docs_hashmap.remove(&doc.hash);
  //   }

  //   // bail if we already have this document
  //   if with_id {
  //     continue;
  //   }

  //   doc.create();

  //   println!("doc");

  //   let mut tokens = string::tokenize(&file.lines);

  //   // Word::update_from_tokens(&mut tokens);
  //   // println!("tok");
  //   // Hit::add_from_tokens(&tokens, doc.id, line_id);

  //   buffer.clear();

  //   line_id += 1;

  // }

  // // for (hash, id) in docs_hashmap {
  // //   let mut doc = Document::with_id(id);

  // //   let mut hits = try!(doc.hits(&state.db.conn));

  // //   for hit in hits {
  // //     let mut word = try!(hit.word(&state.db.conn));

  // //     word.frequency -= 1;
  // //     try!(word.update(&state.db.conn));

  // //     if word.frequency == 0 {
  // //       try!(word.delete(&state.db.conn));
  // //     }

  // //     try!(hit.delete(&state.db.conn));
  // //   }
  // // }

  // println!("bef-finish");
  // // models::finish(&path);

  Ok(())
}
