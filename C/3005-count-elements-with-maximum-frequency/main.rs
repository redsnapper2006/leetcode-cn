use std::collections::HashMap;

impl Solution {
  pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    nums.iter().for_each(|&n| {
      *m.entry(n).or_insert(0) += 1;
    });

    let mut max: i32 = 0;
    let mut cnt: i32 = 0;
    m.iter().for_each(|(_, &v)| {
      if v > max {
        max = v;
        cnt = 1;
      } else if v == max {
        cnt += 1;
      }
    });
    max * cnt
  }
}
