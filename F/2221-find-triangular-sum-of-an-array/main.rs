impl Solution {
  pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
      for j in 0..(nums.len() - 1 - i) {
        nums[j] += nums[j + 1];
        nums[j] %= 10;
      }
    }
    nums[0] % 10
  }
}
