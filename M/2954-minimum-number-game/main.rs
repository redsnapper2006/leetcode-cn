impl Solution {
  pub fn number_game(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();

    (0..nums.len()).step_by(2).for_each(|idx| {
      let t = nums[idx + 1];
      nums[idx + 1] = nums[idx];
      nums[idx] = t;
    });
    nums
  }
}
