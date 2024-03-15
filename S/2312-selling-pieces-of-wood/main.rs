struct Solution {}

impl Solution {
  pub fn selling_wood(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
    let m: usize = m as usize;
    let n: usize = n as usize;

    let mut dp: Vec<Vec<i64>> = vec![vec![0; 201]; 201];
    prices.iter().for_each(|p| {
      dp[p[0] as usize][p[1] as usize] = p[2] as i64;
    });
    let mut idx: usize = 0;
    (1..=m).for_each(|row| {
      (1..=n).for_each(|col| {
        let mut ret: i64 = 0;
        (1..=row / 2).for_each(|r| {
          dp[row][col] = dp[row][col].max(dp[r][col] + dp[row - r][col]);
        });
        (1..=col / 2).for_each(|c| {
          dp[row][col] = dp[row][col].max(dp[row][c] + dp[row][col - c]);
        });
      });
    });

    dp[m][n]
  }

  pub fn selling_wood2(m: i32, n: i32, prices: Vec<Vec<i32>>) -> i64 {
    let m: usize = m as usize;
    let n: usize = n as usize;
    let mut prices = prices;
    prices.sort_by(|a, b| {
      if a[0] == b[0] {
        return a[1].cmp(&b[1]);
      }
      a[0].cmp(&b[0])
    });
    let mut buf: Vec<Vec<i64>> = vec![vec![0; 201]; 201];
    let mut idx: usize = 0;
    (1..=m).for_each(|row| {
      (1..=200).for_each(|idx| {
        buf[row][idx] = buf[row - 1][idx];
      });

      if idx < prices.len() && prices[idx][0] == row as i32 {
        while idx < prices.len() && prices[idx][0] == row as i32 {
          let mut t_max: i64 = 0;
          (1..=prices[idx][1]).for_each(|cc| {
            t_max = t_max.max(buf[row][cc as usize]);
          });
          t_max = t_max.max(prices[idx][2] as i64);
          buf[row][prices[idx][1] as usize] = t_max;
          idx += 1;
        }
      }
    });

    let mut dp: Vec<Vec<i64>> = vec![vec![0; 201]; 201];
    let mut idx: usize = 0;
    (1..=m).for_each(|row| {
      (1..=n).for_each(|col| {
        let mut ret: i64 = 0;
        (1..=row / 2).for_each(|r| {
          ret = ret.max(dp[r][col] + dp[row - r][col]);
        });
        (1..=col / 2).for_each(|c| {
          ret = ret.max(dp[row][c] + dp[row][col - c]);
        });
        ret = ret.max(buf[row][col]);
        dp[row][col] = ret;
      });
    });

    dp[m][n]
  }
}
