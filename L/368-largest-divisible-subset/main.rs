struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut nums = nums;
    nums.sort_unstable();
    nums.iter().for_each(|&v| {
      let sq = (v as f64).sqrt() as i32;
      let mut t: Vec<i32> = vec![];
      (1..=sq).for_each(|q| {
        if v % q == 0 && (m.contains_key(&q)) && (m.get(&q).unwrap().len() > t.len()) {
          t = m.get(&q).unwrap().clone();
        }

        if v % q == 0 && (m.contains_key(&(v / q))) && (m.get(&(v / q)).unwrap().len() > t.len()) {
          t = m.get(&(v / q)).unwrap().clone();
        }
      });
      t.push(v);
      m.insert(v, t);
    });
    let mut ans: Vec<i32> = vec![];
    for (_, v) in m {
      if v.len() > ans.len() {
        ans = v;
      }
    }
    ans
  }
}
