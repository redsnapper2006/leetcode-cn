impl Solution {
  pub fn smallest_index(nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
      let mut sum = 0i32;
      let mut n = nums[i];
      while n > 0 {
        sum += n % 10;
        n /= 10;
      }
      if sum == i as i32 {
        return sum;
      }
    }

    -1
  }
}
