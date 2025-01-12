impl Solution {
  pub fn largest_combination(candidates: Vec<i32>) -> i32 {
    let N: i32 = 25;
    let mut dp: Vec<i32> = vec![0; N as usize];
    candidates.iter().for_each(|&v| {
      (0..N).for_each(|off| {
        dp[off as usize] += if v & (1 << off) > 0 { 1 } else { 0 };
      });
    });
    *dp.iter().max().unwrap()
  }
}
