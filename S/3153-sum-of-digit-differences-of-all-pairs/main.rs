struct Solution {}

impl Solution {
  pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
    let mut dp: Vec<Vec<i64>> = Vec::new();
    let mut v = nums[0];
    while v > 0 {
      let mut c: Vec<i64> = vec![0; 10];
      c[(v % 10) as usize] = 1;
      dp.push(c);
      v /= 10;
    }

    (1..nums.len()).for_each(|idx| {
      let mut v = nums[idx];
      let mut t: usize = 0;
      while v > 0 {
        dp[t][(v % 10) as usize] += 1;
        v /= 10;
        t += 1;
      }
    });

    let mut ans: i64 = 0;
    dp.iter().for_each(|cnt| {
      (0..10).for_each(|idx| {
        ans += cnt[idx] * (nums.len() as i64 - cnt[idx]);
      });
    });
    ans / 2
  }
}

fn main() {
  println!("{}", Solution::sum_digit_differences(vec![13, 23, 12]));
}
