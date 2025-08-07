impl Solution {
  pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = fruits.clone();
    for i in 1..dp.len() {
      dp[i][i] += dp[i - 1][i - 1];
    }

    let cords: Vec<i32> = vec![-1, 0, 1];
    for i in 1..dp.len() {
      let min_c = i.max(fruits[0].len() - i) as i32;
      let cur_c = (i).max(fruits[0].len() - 1 - i);

      for j in cur_c..fruits[0].len() {
        let mut max_v: i32 = 0;
        for cord in cords.iter() {
          let nc = j as i32 + cord;
          if nc < min_c || nc >= fruits[0].len() as i32 {
            continue;
          }
          max_v = max_v.max(dp[i - 1][nc as usize]);
        }
        dp[i][j] += max_v;
      }
    }

    for i in 1..dp[0].len() {
      let min_r = i.max(fruits[0].len() - i) as i32;
      let cur_r = (fruits.len() - 1 - i).max(i);

      for j in cur_r..fruits.len() {
        let mut max_v: i32 = 0;
        for cord in cords.iter() {
          let nr = j as i32 + cord;
          if nr < min_r || nr >= fruits.len() as i32 {
            continue;
          }
          max_v = max_v.max(dp[nr as usize][i - 1]);
        }
        dp[j][i] += max_v;
      }
    }

    dp[fruits.len() - 1][fruits[0].len() - 1]
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::max_collected_fruits(vec![
      vec![1, 2, 3, 4],
      vec![5, 6, 8, 7],
      vec![9, 10, 11, 12],
      vec![13, 14, 15, 16]
    ])
  );

  println!(
    "{}",
    Solution::max_collected_fruits(vec![vec![1, 1], vec![1, 1]])
  );
}
