impl Solution {
  pub fn triangle_type(nums: Vec<i32>) -> String {
    match (
      nums[0] + nums[1] <= nums[2] || nums[0] + nums[2] <= nums[1] || nums[2] + nums[1] <= nums[0],
      nums[0] == nums[1],
      nums[0] == nums[2],
      nums[1] == nums[2],
    ) {
      (true, _, _, _) => "none".to_string(),
      (false, true, true, true) => "equilateral".to_string(),
      (false, true, _, _) => "isosceles".to_string(),
      (false, _, true, _) => "isosceles".to_string(),
      (false, _, _, true) => "isosceles".to_string(),
      (_, _, _, _) => "scalene".to_string(),
    }
  }
}
