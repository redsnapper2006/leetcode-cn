struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
    word.len() as i32 / k
      - *word
        .as_bytes()
        .chunks(k as usize)
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect::<Vec<String>>()
        .iter()
        .fold(HashMap::new(), |mut acc, v| {
          acc.entry(v.clone()).and_modify(|x| *x += 1).or_insert(1);
          acc
        })
        .values()
        .max()
        .unwrap() as i32
  }
}

fn main() {
  println!(
    "{}",
    Solution::minimum_operations_to_make_k_periodic("leetcodeleet".to_string(), 4)
  );
}
