use std::cmp::Ordering;
use super::{WORDS, HITS};
use models::Word;
use util;
use std::collections::hash_set::{HashSet};
use std::collections::hash_map::{HashMap, Entry};

pub struct Cluster {
  id: i32,
  size: i32,
}

impl Cluster {
  pub fn run() {
    let g_words = WORDS.lock().unwrap();
    let g_hits = HITS.lock().unwrap();
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

    let allWords = g_hits.len() as f32;
    let total = words.len() as f32;
    let averageFreq = allWords / total;
    let mut lastFrequentWord = 0;

    for (i,f) in words.iter().enumerate() {
      if f.frequency as f32 <= 100.0 {
        lastFrequentWord = i;

        break;
      }
    }

    // let splitOffWords = words.split_off(lastFrequentWord);
    // let nonFrequentWords: Vec<String> = splitOffWords.iter().map(|x| x.text.clone()).collect();
    // let frequentWords: Vec<String> = words.iter().map(|x| x.text.clone()).collect();
    let mut clusters = HashMap::<&String, Vec<&String>>::new();
    let mut usedWords = HashSet::<&String>::new();
    let mut i = 0;

    for w in &words {
      println!("{:?} {} {}", usedWords.len(), i, w.frequency);
      if usedWords.contains(&w.text) { i += 1; continue; }

      usedWords.insert(&w.text);
      clusters.insert(&w.text, vec!());

      let min_dist = match i {
        y if y < lastFrequentWord => 2,
        _ => 4
      };

      for x in i..words.len() {
        let x_word = words[x];

        if usedWords.contains(&x_word.text) { continue; }

        let dist = util::ld(&w.text, &x_word.text);

        if dist < min_dist {
          usedWords.insert(&x_word.text);
        }
      }

      i += 1;
    }

    // for w in &frequentWords {
    //   if usedWords.contains(w) { continue; }

    //   for x in &frequentWords {
    //     if usedWords.contains(x) { continue; }

    //     let dist = util::ld(w, x);

    //     if dist < 2 {
    //       usedWords.insert(x);

    //       match clusters.entry(w) {
    //         Entry::Occupied(mut entry) => entry.get_mut().push(x),
    //         Entry::Vacant(entry) => {
    //           entry.insert(vec!(x));
    //         }
    //       }
    //     }
    //   }

    //   for x in &nonFrequentWords {
    //     if usedWords.contains(x) { continue; }

    //     let dist = util::ld(w, x);

    //     if dist < 4 {
    //       usedWords.insert(x);

    //       match clusters.entry(w) {
    //         Entry::Occupied(mut entry) => entry.get_mut().push(x),
    //         Entry::Vacant(entry) => {
    //           entry.insert(vec!(x));
    //         }
    //       }
    //     }
    //   }
    // }

    let len = clusters.len();

    for (k,v) in clusters {
      println!("cluster = {}", k);

      for w in v {
        println!("--- {}", w);
      }
    }

    println!("mean: {}, {} {}",  averageFreq, lastFrequentWord, len);
  }
}
