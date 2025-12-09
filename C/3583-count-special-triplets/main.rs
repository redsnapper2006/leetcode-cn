use std::collections::HashMap;

impl Solution {
  pub fn special_triplets(nums: Vec<i32>) -> i32 {
    let mut ans: i64 = 0;

    let mut lm: HashMap<i32, i64> = HashMap::new();
    let mut left: Vec<i64> = vec![0; nums.len()];
    for i in 0..nums.len() {
      left[i] = lm.get(&(nums[i] * 2)).copied().unwrap_or(0);
      *lm.entry(nums[i]).or_insert(0) += 1;
    }

    let mut rm: HashMap<i32, i64> = HashMap::new();
    for i in (0..nums.len()).rev() {
      ans += left[i] * rm.get(&(nums[i] * 2)).copied().unwrap_or(0);
      ans %= 1000000007;
      *rm.entry(nums[i]).or_insert(0) += 1;
    }
    ans as _
  }
}
