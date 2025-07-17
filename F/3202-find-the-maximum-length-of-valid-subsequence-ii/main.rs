impl Solution {
  pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    for i in 0..nums.len() {
      nums[i] %= k;
    }

    let k = k as usize;
    let mut dp: Vec<Vec<(i32, i32)>> = vec![vec![(0, -1); k]; k];

    nums.iter().for_each(|&v| {
      for i in 0..k {
        if i as i32 != v && dp[i][v as usize].1 == i as i32 {
          dp[i][v as usize] = (dp[i][v as usize].0 + 1, v);
        }
      }
      for i in 0..k {
        if i as i32 != v && dp[v as usize][i].1 != v {
          dp[v as usize][i] = (dp[v as usize][i].0 + 1, v);
        }
      }

      dp[v as usize][v as usize] = (dp[v as usize][v as usize].0 + 1, v);
    });

    let mut ans: i32 = 0;
    for i in 0..k {
      for j in 0..k {
        ans = ans.max(dp[i][j].0);
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::maximum_length(vec![1, 2, 3, 4, 5], 2));
}
