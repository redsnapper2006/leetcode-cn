impl Solution {
  pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
    nums.sort_unstable();

    let mut s: usize = 0;
    let mut e: usize = 0;
    let mut ans: usize = nums.len();
    while s < nums.len() && e < nums.len() {
      while e < nums.len() && nums[e] <= nums[s] * k as i64 {
        e += 1;
      }
      ans = ans.min(nums.len() - e + s);
      s += 1;
    }
    ans as _
  }
}
