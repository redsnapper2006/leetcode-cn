struct Solution {}

impl Solution {
  pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    let mut ans: i32 = i32::MAX;
    let mut idx: usize = 0;
    while idx < nums.len() {
      ans = ans.min((nums[idx] - k).abs());
      let mut j: i32 = idx as i32 - 1;
      while j >= 0 && nums[idx] | nums[j as usize] != nums[j as usize] {
        nums[j as usize] |= nums[idx];
        ans = ans.min((nums[j as usize] - k).abs());
        j -= 1;
      }
      idx += 1;
    }
    ans
  }
}

fn main() {
  println!("{}", Solution::minimum_difference(vec![1, 2, 4, 5], 3));
}
