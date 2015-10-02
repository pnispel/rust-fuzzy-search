use util::ld;
use std::collections::hash_map::{HashMap, Entry};
use std::ascii::AsciiExt;
use std::cmp::Ordering;

struct Freq<'a> {
  freq: f32,
  key: &'a String,
}

pub fn sort_by_frequency(words: &mut Vec<String>) -> &mut Vec<String> {
  let mut map = HashMap::<String, u32>::new();
  let mut totalWords = 0.0;

  for s in words.iter() {
    let word = s.to_ascii_lowercase();

    totalWords += 1.0;

    match map.entry(word) {
      Entry::Occupied(mut entry) => { *entry.get_mut() += 1; }
      Entry::Vacant(entry) => { entry.insert(1); }
    }
  }

  let mut sortedWords = Vec::<Freq>::new();

  for (k, v) in map.iter() {
    let floatVal = *v as f32;
    sortedWords.push(Freq {freq: floatVal, key: k});
  }

  words.sort_by(|a, b| {
    let valA = map.get(a).unwrap();
    let valB = map.get(b).unwrap();

    if valA < valB {
      return Ordering::Greater;
    } else if valA > valB {
      return Ordering::Less;
    } else {
      return Ordering::Equal;
    }
  });

  (words)

  // let total = words.len() as f32;
  // let averageFreq = totalWords / total;
  // let mut lastFrequentWord = 0;

  // for (i,f) in words.iter().enumerate() {
  //   if f.freq <= averageFreq {
  //     lastFrequentWord = i;

  //     break;
  //   }
  // }

  // let splitOffWords = words.split_off(lastFrequentWord);
  // let nonFrequentWords: Vec<&String> = splitOffWords.iter().map(|x| x.key).collect();
  // let frequentWords: Vec<&String> = words.iter().map(|x| x.key).collect();

  // for w in &frequentWords {
  //   println!("frequent {}", w);

  //   for x in &frequentWords {
  //     let dist = ld(w, x);

  //     if dist < 2 {
  //       println!("{} = {} {}", dist, w, x);
  //     }
  //   }

  //   for x in &nonFrequentWords {
  //     let dist = ld(w, x);

  //     if dist < 4 {
  //       println!("{} = {} {}", dist, w, x);
  //     }
  //   }
  // }

  // println!("mean: {}, {}",  averageFreq, lastFrequentWord);
}
