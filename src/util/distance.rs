use std::cmp;
use std::string::ToString;

pub fn ld<T>(s1: &T, s2: &T) -> usize where T: ToString {
  let v1: Vec<char> = s1.to_string().chars().collect();
  let v2: Vec<char> = s2.to_string().chars().collect();
  let v1len = v1.len();
  let v2len = v2.len();

  // Early exit if one of the strings is empty
  if v1len == 0 { return v2len; }
  if v2len == 0 { return v1len; }

  fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T{
    cmp::min(v1, cmp::min(v2, v3))
  }
  fn delta(x: char, y: char) -> usize {
    if x == y { 0 } else { 1 }
  }

  let mut column: Vec<usize> = (0..v1len+1).collect();
  for x in 1..v2len+1 {
    column[0] = x;
    let mut lastdiag = x-1;
    for y in 1..v1len+1 {
      let olddiag = column[y];
      column[y] = min3(column[y] + 1,
               column[y-1] + 1,
               lastdiag + delta(v1[y-1], v2[x-1]));
      lastdiag = olddiag;
    }
  }
  column[v1len]
}
