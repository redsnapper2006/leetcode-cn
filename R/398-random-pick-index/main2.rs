use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

struct Solution {
  map: HashMap<i32, Vec<i32>>,
}

impl Solution {
  fn new(mut nums: Vec<i32>) -> Self {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    nums.iter().enumerate().for_each(|(i, x)| {
      let entry = map.entry(*x).or_insert(vec![]);
      (*entry).push(i as i32);
    });
    Solution { map }
  }

  fn pick(&self, target: i32) -> i32 {
    let vec = self.map.get(&target).unwrap();
    let rand = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .subsec_micros();
    vec[rand as usize % vec.len()]
  }
}
