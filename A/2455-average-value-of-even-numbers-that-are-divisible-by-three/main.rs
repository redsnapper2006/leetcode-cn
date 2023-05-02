struct Solution {}

impl Solution {
  pub fn average_value(nums: Vec<i32>) -> i32 {
    let a = nums
      .into_iter()
      .filter(|&v| v % 6 == 0)
      .collect::<Vec<i32>>();
    match a.len() {
      0 => 0,
      _ => a.iter().sum::<i32>() / a.len() as i32,
    }
  }
}
