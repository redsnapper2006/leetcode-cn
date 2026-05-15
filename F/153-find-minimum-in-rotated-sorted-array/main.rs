impl Solution {
  pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut s: usize = 0;
    let mut e: usize = nums.len() - 1;
    while s < e {
      let m = s + (e - s) / 2;
      if nums[m] < nums[e] {
        e = m;
      } else {
        s = m + 1;
      }
    }
    nums[s]
  }
}
