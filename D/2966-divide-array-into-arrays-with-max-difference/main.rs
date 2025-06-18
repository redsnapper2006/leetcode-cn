impl Solution {
  pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut ans : Vec<Vec<i32>> = vec![];
    for idx in   (2..nums.len()).step_by(3) {
      if nums[idx] - nums[idx-2] > k {
        return vec![];
      }
      ans.push(nums[idx-2..=idx].to_vec());
    }

    ans
  }
}
