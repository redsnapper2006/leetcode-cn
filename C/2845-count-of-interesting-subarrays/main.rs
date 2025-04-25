impl Solution {
  pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
    let mut ans: i64 = 0;
    let mut dp: i32 = 0;
    let mut m: Vec<i64> = vec![0; (nums.len() + 1).min(modulo as usize)];
    m[0] = 1;
    (0..nums.len()).for_each(|idx| {
      dp += if nums[idx] % modulo == k { 1 } else { 0 };

      let c = (dp - k) % modulo;
      if c >= 0 {
        ans += m[c as usize];
      }

      m[(dp % modulo) as usize] += 1;
    });
    ans
  }
}

struct Solution {}
fn main() {
  println!(
    "{}",
    Solution::count_interesting_subarrays(vec![3, 2, 4], 2, 1)
  );

  println!(
    "{}",
    Solution::count_interesting_subarrays(vec![3, 1, 9, 6], 3, 0)
  );
}
