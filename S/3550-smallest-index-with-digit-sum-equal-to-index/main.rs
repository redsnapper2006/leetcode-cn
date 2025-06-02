impl Solution {
  pub fn smallest_index(nums: Vec<i32>) -> i32 {
    let mut idx :usize = 0;
    while idx < nums.len() {
      let mut sum : i32 = 0;
      let mut n = nums[idx];
      while n > 0 {
        sum += n % 10;
        n /= 10;
      }
      if sum == idx as i32 {
        return sum;
      }

      idx += 1;
    }
    -1
  }
}
