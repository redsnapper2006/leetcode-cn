use std::collections::HashMap;

impl Solution {
  pub fn min_distinct_freq_pair(nums: Vec<i32>) -> Vec<i32> {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    for &i in &nums {
      *cnt.entry(i).or_insert(0) += 1;
    }
    let mut rev: HashMap<i32, i32> = HashMap::new();
    for (k, v) in cnt {
      rev.entry(v).and_modify(|e| *e = (*e).min(k)).or_insert(k);
    }

    let mut min1 = (i32::MAX, i32::MAX); // (freq, val)
    let mut min2 = (i32::MAX, i32::MAX);
    for (freq, val) in rev {
      if freq < min1.0 {
        min2 = min1;
        min1 = (freq, val);
      } else if freq < min2.0 {
        min2 = (freq, val);
      }
    }

    if min2.0 == i32::MAX {
      vec![-1, -1]
    } else {
      vec![min1.0, min2.0]
    }
  }
}
