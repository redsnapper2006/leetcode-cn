impl Solution {
  pub fn find_valid_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut right: Vec<bool> = vec![false; nums.len()];
    let mut right_mx: i32 = *nums.last().unwrap() - 1;
    for i in (0..nums.len()).rev() {
      if nums[i] > right_mx {
        right[i] = true;
        right_mx = nums[i];
      }
    }

    let mut left_mx: i32 = nums[0] - 1;
    let mut ans: Vec<i32> = vec![];
    for i in 0..nums.len() {
      if nums[i] > left_mx || right[i] {
        ans.push(nums[i]);
      }
      left_mx = left_mx.max(nums[i]);
    }
    ans
  }
}
