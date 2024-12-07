struct Solution {}

impl Solution {
  pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
    let n = n as usize;
    let k = k as usize;
    let cords: Vec<Vec<i32>> = vec![
      vec![-1, -2],
      vec![-2, -1],
      vec![-2, 1],
      vec![-1, 2],
      vec![1, 2],
      vec![2, 1],
      vec![2, -1],
      vec![1, -2],
    ];
    let mut dp: Vec<Vec<Vec<f64>>> = vec![vec![vec![0.0; n]; n]; 2];

    dp[1][row as usize][column as usize] = 1.0;
    (0..k).for_each(|step| {
      let cur = step % 2;
      let prev = (step + 1) % 2;

      dp[cur] = vec![vec![0.0; n]; n];
      (0..n).for_each(|r| {
        (0..n).for_each(|c| {
          cords.iter().for_each(|cord| {
            let nr = r as i32 + cord[0];
            let nc = c as i32 + cord[1];
            if nr < 0 || nr >= n as i32 || nc < 0 || nc >= n as i32 {
              return;
            }
            dp[cur][nr as usize][nc as usize] += dp[prev][r][c] / 8.0;
          });
        });
      });
    });

    let mut ans: f64 = 0.0;
    (0..n).for_each(|r| {
      (0..n).for_each(|c| {
        ans += dp[(k + 1) % 2][r][c];
      });
    });
    ans
  }
}

fn main() {
  println!("{}", Solution::knight_probability(3, 2, 0, 0));
}
