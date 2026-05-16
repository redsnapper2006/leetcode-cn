impl Solution {
  pub fn find_min(nums: Vec<i32>) -> i32 {
    let (mut s, mut e) = (0, nums.len() - 1);
    while s < e {
      let m = s + (e - s) / 2;
      if nums[m] < nums[e] {
        e = m;
      } else if nums[m] > nums[e] {
        s = m + 1;
      } else {
        e -= 1;
      }
    }
    nums[s]
  }
}
