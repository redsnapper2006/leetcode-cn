struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    for n in nums {
      let mut c = m.entry(n).or_insert(0);
      *c += 1;
    }

    let count: Vec<i32> = m.into_values().collect();
    let mut ret: i32 = 0;
    (0..count.len()).for_each(|i1| {
      (i1 + 1..count.len()).for_each(|i2| {
        (i2 + 1..count.len()).for_each(|i3| {
          ret += count[i1] * count[i2] * count[i3];
        })
      })
    });
    ret
  }
}
