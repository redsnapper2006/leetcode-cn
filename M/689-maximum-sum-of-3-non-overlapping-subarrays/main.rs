struct Solution {}

impl Solution {
  pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut end: i32 = nums.len() as i32 - 1;

    let mut buf: Vec<Vec<(i32, Vec<usize>)>> = vec![vec![(0, Vec::new()); nums.len()]; 3];
    let mut sum: i32 = 0;
    let kk: usize = k as usize;
    while end >= 0 {
      let idx: usize = end as usize;
      sum += nums[idx];
      let prev_k: usize = idx + kk;
      let prev: usize = idx + 1;
      if idx < nums.len() - kk {
        sum -= nums[prev_k];
      }
      if idx == nums.len() - kk {
        buf[0][idx] = (sum, vec![idx]);
      }
      if idx < nums.len() - kk {
        if buf[0][prev].0 <= sum {
          buf[0][idx] = (sum, vec![idx]);
        } else {
          buf[0][idx] = buf[0][prev].clone();
        }
      }
      if idx == nums.len() - 2 * kk {
        buf[1][idx] = (sum + buf[0][prev_k].0, vec![idx, buf[0][prev_k].1[0]]);
      }
      if idx < nums.len() - 2 * kk {
        if sum + buf[0][idx + kk].0 >= buf[1][idx + 1].0 {
          buf[1][idx] = (sum + buf[0][idx + kk].0, vec![idx, buf[0][idx + kk].1[0]])
        } else {
          buf[1][idx] = buf[1][idx + 1].clone();
        }
      }

      if idx == nums.len() - 3 * kk {
        buf[2][idx] = (
          sum + buf[1][idx + kk].0,
          vec![idx, buf[1][idx + kk].1[0], buf[1][idx + kk].1[1]],
        );
      }
      if idx < nums.len() - 3 * kk {
        if sum + buf[1][idx + kk].0 >= buf[2][idx + 1].0 {
          buf[2][idx] = (
            sum + buf[1][idx + kk].0,
            vec![idx, buf[1][idx + kk].1[0], buf[1][idx + kk].1[1]],
          )
        } else {
          buf[2][idx] = buf[2][idx + 1].clone();
        }
      }

      end -= 1;
    }

    buf[2][0].1.iter().map(|&v| v as i32).collect::<Vec<i32>>()
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2)
  );
  println!(
    "{:?}",
    Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2)
  );
  println!(
    "{:?}",
    Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 2, 2, 2, 2], 2)
  );
}
