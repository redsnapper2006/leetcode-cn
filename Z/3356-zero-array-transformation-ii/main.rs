impl Solution {
  pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let mut cnt: i32 = 0;
    let mut idx: usize = 0;
    while idx < nums.len() {
      if nums[idx] > 0 {
        cnt += 1;
      }
      idx += 1;
    }
    if cnt == 0 {
      return 0;
    }
    let mut l: i32 = 0;
    let mut r: i32 = queries.len() as i32 - 1;
    while l <= r {
      let m = l + (r - l) / 2;
      let mut diff: Vec<i32> = vec![0; nums.len() + 1];
      (0..=m as usize).for_each(|idx| {
        diff[queries[idx][0] as usize] += queries[idx][2];
        diff[queries[idx][1] as usize + 1] -= queries[idx][2];
      });
      let mut valid: bool = true;
      let mut sum: i32 = 0;
      for ii in 0..nums.len() {
        sum += diff[ii];
        if sum < nums[ii] {
          valid = false;
          break;
        }
      }
      if valid {
        r = m - 1;
      } else {
        l = m + 1;
      }
    }
    if l == queries.len() as i32 {
      -1
    } else {
      l as i32 + 1
    }
  }

  pub fn min_zero_array2(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<i32> = nums;
    let mut cnt: i32 = 0;
    let mut idx: usize = 0;
    while idx < dp.len() {
      if dp[idx] > 0 {
        cnt += 1;
      }
      idx += 1;
    }
    if cnt == 0 {
      return 0;
    }

    let mut q: usize = 0;
    while q < queries.len() {
      (queries[q][0]..=queries[q][1]).for_each(|idx| {
        let idx = idx as usize;
        if dp[idx] <= 0 {
          return;
        }

        dp[idx] -= queries[q][2];
        if dp[idx] <= 0 {
          cnt -= 1;
        }
      });

      if cnt == 0 {
        return q as i32 + 1;
      }

      q += 1;
    }
    -1
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::min_zero_array(
      vec![2, 0, 2],
      vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
    )
  );

  println!(
    "{}",
    Solution::min_zero_array(vec![4, 3, 2, 1], vec![vec![1, 3, 2], vec![0, 2, 1]])
  );
  println!(
    "{}",
    Solution::min_zero_array(
      vec![0],
      vec![
        vec![0, 0, 2],
        vec![0, 0, 4],
        vec![0, 0, 4],
        vec![0, 0, 3],
        vec![0, 0, 5]
      ]
    )
  );
}
