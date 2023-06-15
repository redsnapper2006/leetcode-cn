struct Solution {}

impl Solution {
  pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let mut start: usize = 0;
    let mut end: usize = nums.len() - 1;

    let mut sum: i64 = 0;
    while start < end {
      let mut r = nums[end];
      let mut l = nums[start] as i64;
      while r > 0 {
        l *= 10;
        r /= 10;
      }
      sum += l + nums[end] as i64;
      start += 1;
      end -= 1;
    }
    match start == end {
      true => sum + nums[end] as i64,
      false => sum,
    }
  }
}
