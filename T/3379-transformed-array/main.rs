impl Solution {
  pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = nums.clone();

    (0..nums.len()).for_each(|idx| {
      let mut off = (idx as i32 + nums[idx]) % nums.len() as i32;
      while off < 0 {
        off += nums.len() as i32;
        off %= nums.len() as i32;
      }

      ans[idx] = nums[off as usize];
    });
    ans
  }
}
