struct Solution {}

impl Solution {
  pub fn h_index(citations: Vec<i32>) -> i32 {
    let mut citations = citations;
    citations.sort();
    let mut h = 0;
    (0..citations.len()).rev().for_each(|i| {
      if citations[i] > h as i32 {
        h += 1;
      }
    });
    h
  }
}
