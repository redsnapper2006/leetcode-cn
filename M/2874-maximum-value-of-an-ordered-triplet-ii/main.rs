impl Solution {
  pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
    let mut ans: i64 = 0;
    let mut max_num: i64 = 0;
    let mut max_diff: i64 = 0;
    nums.iter().for_each(|&vv| {
      let v = vv as i64;
      ans = ans.max(max_diff * v);
      max_diff = max_diff.max(max_num - v);
      max_num = max_num.max(v);
    });
    ans
  }
}
