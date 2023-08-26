struct Solution {}

impl Solution {
  pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.len() == 0 {
      return Vec::new();
    }
    let mut start: i32 = nums[0];
    let mut end: i32 = nums[0];
    let mut result: Vec<String> = Vec::new();
    (1..nums.len()).for_each(|i| {
      if nums[i] == end + 1 {
        end = nums[i];
      } else {
        if start == end {
          result.push(start.to_string());
        } else {
          result.push(format!("{}->{}", start, end));
        }
        start = nums[i];
        end = nums[i];
      }
    });
    if start == end {
      result.push(start.to_string());
    } else {
      result.push(format!("{}->{}", start, end));
    }
    result
  }
}
