struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn fair_candy_swap(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut suma = 0;
    let mut sumb = 0;
    let mut m: HashSet<i32> = HashSet::new();
    for i in &a {
      suma += i;
    }
    for i in &b {
      sumb += i;
      m.insert(*i);
    }
    let diff = (sumb - suma) / 2;
    let mut r1 = 0;
    let mut r2 = 0;
    for i in &a {
      let t = i + diff;
      if m.contains(&t) {
        r1 = *i;
        r2 = t;
        break;
      }
    }
    vec![r1, r2]
  }
}

fn main() {
  println!("{}", Solution::fair_candy_swap(vec![0; 5], vec![0; 5]));
}
