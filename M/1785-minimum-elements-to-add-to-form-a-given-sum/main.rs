struct Solution {}

impl Solution {
  pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
    (((goal as i64 - nums.into_iter().map(|x| x as i64).sum::<i64>()).abs() + limit as i64 - 1)
      / limit as i64) as i32
  }
}
