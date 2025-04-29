impl Solution {
  pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let max_num = *nums.iter().max().unwrap();
    let mut ans: i64 = 0;
    let mut l: usize = 0;
    let mut r: usize = 0;
    let mut cnt: i32 = 0;
    while r < nums.len() {
      cnt += if nums[r] == max_num { 1 } else { 0 };

      while cnt >= k && l <= r {
        ans += (nums.len() - r) as i64;
        cnt -= if nums[l] == max_num { 1 } else { 0 };
        l += 1;
      }

      r += 1;
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2));
}
