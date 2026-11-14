impl Solution {
  pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
    let (mut inc, mut dec) = (1i32, 1i32);
    let mut ans = 1i32;
    for i in 1..nums.len() {
      if nums[i] > nums[i - 1] {
        inc += 1;
        dec = 1;
      } else if nums[i] < nums[i - 1] {
        inc = 1;
        dec += 1;
      } else {
        inc = 1;
        dec = 1;
      }
      ans = ans.max(inc).max(dec);
    }

    ans
  }
}
