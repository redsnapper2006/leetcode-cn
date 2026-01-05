impl Solution {
  pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
    let mut prev: (i64, usize) = (nums[0] as i64, 0);

    let mut ans: i64 = 0;
    for i in 1..nums.len() {
      if nums[i] as i64 > prev.0 {
        ans += (i - prev.1) as i64 * prev.0;
        prev = (nums[i] as i64, i);
      }
    }

    ans + (nums.len() - 1 - prev.1) as i64 * prev.0
  }
}
