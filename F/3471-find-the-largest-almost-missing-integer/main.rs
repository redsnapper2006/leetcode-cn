impl Solution {
  pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    if k == nums.len() as i32 {
      return *nums.iter().max().unwrap();
    }
    let mut dp :Vec<i32> = vec![0;51];
    nums.iter().enumerate().for_each(|(idx, &v) | {
      dp[v as usize] += (idx+1).min(k as usize).min(nums.len()-idx) as i32;
    });

    for i in (0..51).rev() {
      if dp[i] == 1  {
        return i as _;
      }
    }
    -1
  }
}
