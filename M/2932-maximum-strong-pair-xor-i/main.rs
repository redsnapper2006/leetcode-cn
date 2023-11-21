struct Solution {}

impl Solution {
  pub fn maximum_strong_pair_xor(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut max: i32 = 0;
    (0..nums.len() - 1).for_each(|i| {
      let mut j = i;
      while j < nums.len() && nums[i] * 2 >= nums[j] {
        max = max.max(nums[i] ^ nums[j]);
        j += 1;
      }
    });
    max
  }
}
