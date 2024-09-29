struct Solution {}

impl Solution {
  pub fn can_alice_win(nums: Vec<i32>) -> bool {
    let total = nums.iter().sum::<i32>();
    let single = nums
      .into_iter()
      .filter(|&x| x < 10)
      .collect::<Vec<i32>>()
      .iter()
      .sum::<i32>();
    single > total - single || total - single > single
  }
}
