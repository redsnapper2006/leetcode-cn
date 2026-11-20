impl Solution {
  pub fn max_difference(nums: Vec<i32>) -> i64 {
    let nums = nums.iter().map(|&x| x as i64).collect::<Vec<i64>>();
    let n = nums.len();
    let mut dp: Vec<Vec<i64>> = vec![vec![i64::MIN; 2]; nums.len()];

    fn recur(dp: &mut Vec<Vec<i64>>, nums: &Vec<i64>, idx: usize, n: usize, round: usize) -> i64 {
      if idx >= n {
        return 0;
      }
      let round = round % 2;

      if dp[idx][round] != i64::MIN {
        return dp[idx][round];
      }

      let diff1 = if nums[idx] % 3 == 0 {
        nums[idx] - recur(dp, nums, idx + 1, n, round + 1)
      } else {
        nums[idx] + recur(dp, nums, idx + 1, n, round)
      };

      let mut diff2 = i64::MIN;
      if idx + 1 < n {
        diff2 = if (nums[idx] + nums[idx + 1]) % 3 == 0 {
          nums[idx] + nums[idx + 1] - recur(dp, nums, idx + 2, n, round + 1)
        } else {
          nums[idx] + nums[idx + 1] + recur(dp, nums, idx + 2, n, round)
        };
      }

      dp[idx][round] = diff1.max(diff2);
      dp[idx][round]
    }

    recur(&mut dp, &nums, 0, n, 0);
    dp[0][0]
  }
}
