struct Solution {}

impl Solution {
  pub fn special_perm(nums: Vec<i32>) -> i32 {
    let mut dp: Vec<Vec<i64>> = vec![vec![-1; nums.len()]; (1 << nums.len()) - 1];

    fn dfs(nums: &Vec<i32>, dp: &mut Vec<Vec<i64>>, s: usize, i: usize) -> i64 {
      if s == 0 {
        return 1;
      }
      if dp[s][i] != -1 {
        return dp[s][i];
      }

      let mut res: i64 = 0;
      nums.iter().enumerate().for_each(|(j, v)| {
        if ((s >> j) & 1) > 0 && (nums[i] % v == 0 || v % nums[i] == 0) {
          res += dfs(nums, dp, s ^ (1 << j), j);
        }
      });

      dp[s][i] = res;
      res
    }
    let mut ans: i64 = 0;
    (0..nums.len()).for_each(|idx| {
      ans += dfs(&nums, &mut dp, ((1 << nums.len()) - 1) ^ (1 << idx), idx);
      ans %= 1000000007;
    });
    ans as i32
  }
}

fn main() {
  println!("{}", Solution::special_perm(vec![3, 6, 2]));
  println!(
    "{}",
    Solution::special_perm(vec![
      1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192
    ])
  );
}
