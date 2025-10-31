impl Solution {
  pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut buf: i128 = 0;
    nums.iter().for_each(|&v| {
      buf ^= (1 << v);
    });
    let mut ans: Vec<i32> = vec![];
    for i in 0..nums.len() - 2 {
      if buf & (1 << i) == 0 {
        ans.push(i as i32);
      }
    }
    ans
  }

  pub fn get_sneaky_numbers2(nums: Vec<i32>) -> Vec<i32> {
    let mut dp: Vec<i32> = vec![0; nums.len() - 2];
    nums.iter().for_each(|&v| {
      dp[v as usize] += 1;
    });
    let mut ans: Vec<i32> = Vec::new();
    dp.iter().enumerate().for_each(|(idx, &v)| {
      if v == 2 {
        ans.push(idx as i32);
      }
    });
    ans
  }
}
