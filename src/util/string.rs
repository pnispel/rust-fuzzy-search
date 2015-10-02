use regex::Regex;
use std::ascii::AsciiExt;
use std::collections::HashSet;
use util::STOP_WORDS;

pub fn tokenize (line: &String) -> Vec<String> {
  let stop_words: HashSet<&str> = STOP_WORDS.iter().cloned().collect();
  let line_slice: &str = &line[..];
  let splitRegex = Regex::new(r"[^a-zA-Z0-9’']+").unwrap();
  let mut found = HashSet::<String>::new();

  let splits = splitRegex.split(line_slice)
    .map(|x| x.to_ascii_lowercase())
    .map(|x| {
      let y = x.replace("'", "");
      (y.replace("’", ""))
    })
    .filter(|x| {
      // if x.len() != 0 && !found.contains(x) {
      //   found.insert(x.clone());

      //   return true;
      // }

      // return false;
      (x.len() != 0 && !stop_words.contains(&x[..]))
    }).collect();

  (splits)
}

pub fn without_stops (tokens: &Vec<String>) -> Vec<String> {
  let stop_words: HashSet<&str> = STOP_WORDS.iter().cloned().collect();
  let mut ret = Vec::<String>::new();

  for token in tokens {
    if !stop_words.contains(&token[..]) {
      ret.push(token.clone())
    }
  }

  (ret)
}
