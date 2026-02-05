impl Solution {
  pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = nums.clone();

    (0..nums.len()).for_each(|idx| {
      let mut v = nums[idx];
      v %= nums.len() as i32;
      v += if v < 0 { nums.len() as i32 } else { 0 };
      ans[idx] = nums[(idx + v as usize) % nums.len()];
    });
    ans
  }
}
