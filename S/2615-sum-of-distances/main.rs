use std::collections::HashMap;

impl Solution {
  pub fn distance(nums: Vec<i32>) -> Vec<i64> {
    let mut left: Vec<i64> = vec![0; nums.len()];
    let mut left_map: HashMap<i32, (usize, i64)> = HashMap::new();
    let mut right: Vec<i64> = vec![0; nums.len()];
    let mut right_map: HashMap<i32, (usize, i64)> = HashMap::new();
    for i in 0..nums.len() {
      let v = left_map.entry(nums[i]).or_insert((i, 0));
      left[i] = v.1 * ((i - v.0) as i64) + left[v.0];
      v.0 = i;
      v.1 += 1;

      let v = right_map.entry(nums[nums.len() - 1 - i]).or_insert((nums.len() - 1 - i, 0));
      right[nums.len() - 1 - i] = v.1 * ((v.0 - (nums.len() - 1 - i)) as i64) + right[v.0];
      v.0 = nums.len() - 1 - i;
      v.1 += 1;
    }

    let mut ans: Vec<i64> = vec![0; nums.len()];
    for i in 0..nums.len() {
      ans[i] = left[i] + right[i];
    }
    ans
  }
}
