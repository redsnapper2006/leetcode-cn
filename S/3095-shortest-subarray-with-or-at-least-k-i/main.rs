struct Solution {}

impl Solution {
  pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let N: usize = 8;
    let nn = nums
      .iter()
      .map(|&v| {
        let mut vv: Vec<i32> = vec![0; N];
        let mut v = v;
        let mut idx: usize = 0;
        while v > 0 {
          vv[idx] = v % 2;
          v /= 2;
          idx += 1;
        }
        vv
      })
      .collect::<Vec<Vec<i32>>>();
    let mut dp: Vec<i32> = vec![0; N];
    let mut kb: Vec<i32> = vec![0; N];
    let mut kk = k;
    let mut idx: usize = 0;
    while kk > 0 {
      kb[idx] = kk % 2;
      kk /= 2;
      idx += 1;
    }

    fn is_match(dp: &Vec<i32>, kb: &Vec<i32>, N: usize) -> bool {
      let mut ans: bool = false;
      let mut idx: i32 = N as i32 - 1;
      while idx >= 0 {
        if kb[idx as usize] == 0 {
          if dp[idx as usize] > 0 {
            return true;
          }
        } else {
          if dp[idx as usize] == 0 {
            return false;
          }
        }
        idx -= 1;
      }
      true
    }
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut ans: i32 = nums.len() as i32 + 1;

    while end < nums.len() {
      (0..N).for_each(|idx| {
        dp[idx] += nn[end][idx];
      });
      while start <= end && is_match(&dp, &kb, N) {
        ans = ans.min((end - start) as i32 + 1);
        (0..N).for_each(|idx| {
          dp[idx] -= nn[start][idx];
        });
        start += 1;
      }
      end += 1;
    }
    if ans == nums.len() as i32 + 1 {
      -1
    } else {
      ans
    }
  }
}
