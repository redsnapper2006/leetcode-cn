struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut m: HashMap<i32, usize> = HashMap::new();

    let mut ans: bool = false;
    nums.iter().enumerate().for_each(|(idx, &v)| {
      m.entry(v)
        .and_modify(|x| {
          ans |= (idx - *x) as i32 <= k;
          *x = idx;
        })
        .or_insert(idx);
    });
    ans
  }
}
