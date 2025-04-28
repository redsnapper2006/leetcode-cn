impl Solution {
  pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
    let mut sum: i64 = 0;
    let mut cnt: i64 = 0;
    let mut r: usize = 0;
    let mut l: usize = 0;
    let mut ans: i64 = 0;
    while r < nums.len() {
      sum += nums[r] as i64;
      cnt += 1;

      while sum * cnt >= k && l <= r {
        ans += (r - l) as i64;
        sum -= nums[l] as i64;
        cnt -= 1;
        l += 1;
      }

      r += 1;
    }

    ans + (r - l) as i64 * (r - l + 1) as i64 / 2
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::count_subarrays(vec![2, 1, 4, 3, 5], 10));
  println!("{}", Solution::count_subarrays(vec![1, 1, 1], 5));
}
