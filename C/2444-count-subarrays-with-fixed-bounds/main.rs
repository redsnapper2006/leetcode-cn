impl Solution {
  pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
    let mut base: i64 = -1;
    let mut r: usize = 0;
    let mut ans: i64 = 0;
    let mut min_p: i64 = -1;
    let mut max_p: i64 = -1;

    while r < nums.len() {
      if nums[r] < min_k || nums[r] > max_k {
        base = r as i64;
        min_p = base;
        max_p = base;
        r += 1;
        continue;
      }

      if nums[r] == min_k {
        min_p = r as i64;
      }
      if nums[r] == max_k {
        max_p = r as i64;
      }
      ans += min_p.min(max_p) - base;
      r += 1;
    }
    ans
  }
}
