impl Solution {
  pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    nums.iter().enumerate().fold(i32::MAX, |ans, (idx, &v)| {
      if v == target {
        ans.min((idx as i32 - start).abs())
      } else {
        ans
      }
    })
  }
}
