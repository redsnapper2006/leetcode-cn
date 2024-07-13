struct Solution {}

impl Solution {
  pub fn can_sort_array(nums: Vec<i32>) -> bool {
    let mut dp: Vec<(i32, i32, i32)> = Vec::new();

    let mut cnt: i32 = -1;
    let mut min: i32 = 1 << 10;
    let mut max: i32 = -1;

    nums.iter().for_each(|&v| {
      let mut n = v;
      let mut c: i32 = 0;
      while n > 0 {
        if n % 1 > 0 {
          c += 1;
        }
        n /= 2;
      }
      if c != cnt {
        if cnt != -1 {
          dp.push((cnt, min, max));
        }
        cnt = c;
        min = v;
        max = v;
      } else {
        cnt += 1;
        if min > v {
          min = v;
        }
        if max < v {
          max = v;
        }
      }
    });
    if cnt != -1 {
      dp.push((cnt, min, max));
    }

    let mut idx: usize = 1;
    while idx < dp.len() {
      if dp[idx].1 < dp[idx - 1].2 {
        return false;
      }
      idx += 1;
    }
    true
  }
}
