struct Solution {}

impl Solution {
  pub fn maximum_or(nums: Vec<i32>, k: i32) -> i64 {
    let mut suffix: Vec<i64> = vec![0; nums.len()];
    (0..nums.len() - 1).rev().for_each(|idx| {
      suffix[idx] = suffix[idx + 1] | nums[idx + 1] as i64;
    });
    let mut ans: i64 = 0;
    let mut pre: i64 = 0;
    (0..nums.len()).for_each(|idx| {
      ans = ans.max(pre | (nums[idx] as i64) << (k as i64) | suffix[idx]);
      pre |= nums[idx] as i64;
    });
    ans
  }

  pub fn maximum_or2(nums: Vec<i32>, k: i32) -> i64 {
    let mut max_bit: i32 = -1;
    let mut target: Vec<usize> = vec![];
    let mut dp: Vec<i64> = vec![0; 64];
    nums.iter().enumerate().for_each(|(idx, &v)| {
      let mut off: i32 = 31;
      while off >= 0 {
        if v & (1 << off) == 0 {
          off -= 1;
        } else {
          if off > max_bit {
            max_bit = off;
            target = vec![idx];
          } else if off == max_bit {
            target.push(idx);
          }
          break;
        }
      }
      let mut vv = v as i64;
      let mut off: usize = 0;
      while vv > 0 {
        dp[off] += vv % 2;
        vv /= 2;
        off += 1;
      }
    });

    let mut ans: i64 = 0;
    target.iter().for_each(|&v| {
      let mut dc = dp.clone();
      let mut vv = nums[v] as i64;
      let mut off: usize = 0;
      while vv > 0 {
        dc[off] -= vv % 2;
        dc[off + k as usize] += vv % 2;
        vv /= 2;
        off += 1;
      }

      let mut mx: i64 = 0;
      (0..63).rev().for_each(|idx| {
        mx = mx * 2 + if dc[idx] > 0 { 1 } else { 0 };
      });
      ans = ans.max(mx);
    });
    ans
  }
}

fn main() {
  println!("{}", Solution::maximum_or(vec![12, 9], 1));
  println!("{}", Solution::maximum_or(vec![8, 1, 2], 2));
  println!("{}", Solution::maximum_or(vec![1], 9));
}
