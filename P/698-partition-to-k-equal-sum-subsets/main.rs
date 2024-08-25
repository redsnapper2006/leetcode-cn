struct Solution {}

impl Solution {
  pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let mut nums = nums;
    nums.sort_unstable();

    let sum = nums.iter().sum::<i32>();
    if sum % k > 0 || nums[0] > sum / k {
      return false;
    }

    let size = nums.len();
    let mut dp: Vec<bool> = vec![true; 1 << size];
    fn dfs(
      nums: &Vec<i32>,
      size: usize,
      visited: usize,
      target: i32,
      sum: i32,
      dp: &mut Vec<bool>,
    ) -> bool {
      if visited == (1 << size) - 1 {
        return true;
      }

      if !dp[visited] {
        return dp[visited];
      }
      dp[visited] = false;

      let mut idx: usize = 0;
      while idx < size {
        if (visited & 1 << idx) > 0 || sum + nums[idx] > target {
          idx += 1;
          continue;
        }

        if dfs(
          nums,
          size,
          visited | 1 << idx,
          target,
          (sum + nums[idx]) % target,
          dp,
        ) {
          return true;
        }

        idx += 1;
      }
      false
    }
    dfs(&nums, size, 0, sum / k, 0, &mut dp)
  }
}

fn main() {
  println!(
    "{}",
    Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4)
  );

  println!(
    "{}",
    Solution::can_partition_k_subsets(vec![3, 3, 10, 2, 6, 5, 10, 6, 8, 3, 2, 1, 6, 10, 7, 2], 6)
  );
  println!(
    "{}",
    Solution::can_partition_k_subsets(
      vec![
        5309, 7138, 779, 8949, 8568, 2250, 1794, 6539, 4948, 7189, 4270, 9866, 5867, 2112, 9176,
        3162
      ],
      2
    )
  );
}
