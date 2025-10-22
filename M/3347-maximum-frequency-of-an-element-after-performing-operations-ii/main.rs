impl Solution {
  pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    let mut left: usize = 0;
    let mut left2: usize = 0;
    let mut cnt: i32 = 0;
    let mut right: usize = 0;
    let mut ans: i32 = 0;
    for i in 0..nums.len() {
      while nums[left2] < nums[i] - 2 * k {
        left2 += 1;
      }
      ans = ans.max(num_operations.min((i - left2) as i32 + 1));
      cnt += 1;
      if i < nums.len() - 1 && nums[i] == nums[i + 1] {
        continue;
      }
      while nums[left] < nums[i] - k {
        left += 1;
      }
      while right < nums.len() && nums[right] <= nums[i] + k {
        right += 1;
      }
      ans = ans.max((cnt + num_operations).min((right - left) as i32));
      cnt = 0;
    }
    ans
  }
}
struct Solution {}

fn main() {
  println!("{}", Solution::max_frequency(vec![1, 4, 5], 1, 2));
  println!("{}", Solution::max_frequency(vec![5, 11, 20, 20], 5, 1));

  println!("{}", Solution::max_frequency(vec![1, 90], 76, 1));
  println!("{}", Solution::max_frequency(vec![58, 80, 5], 58, 2));
  println!("{}", Solution::max_frequency(vec![9], 0, 0));
}
