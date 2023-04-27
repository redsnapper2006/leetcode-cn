struct Solution {}

impl Solution {
  pub fn find_max_k(nums: Vec<i32>) -> i32 {
    let mut dp: [i32; 2001] = [0; 2001];
    nums.iter().for_each(|&v| {
      dp[v as usize + 1000] = 1;
    });
    let mut ret: i32 = -1;
    let mut i: usize = 0;
    while i < 1000 {
      if dp[i] == 1 && dp[2000 - i] == 1 {
        return 1000 - i as i32;
      }
      i += 1;
    }
    -1
  }
}
