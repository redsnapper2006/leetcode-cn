impl Solution {
  pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    let (zero, one, limit) = (zero as usize, one as usize, limit as usize);
    let mut dp: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; one + 1]; zero + 1]; 2];
    dp[0][0][0] = 1;
    dp[1][0][0] = 1;
    dp[0][1][0] = 1;
    dp[1][0][1] = 1;

    for cnt in 2..=(zero + one) {
      for zcnt in 0..=cnt {
        let ocnt = cnt - zcnt;
        if zcnt > zero || ocnt > one {
          continue;
        }
        let vz = if zcnt > 0 {
          dp[0][zcnt - 1][ocnt] + dp[1][zcnt - 1][ocnt]
        } else {
          0
        } - if zcnt > limit {
          dp[1][zcnt - limit - 1][ocnt]
        } else {
          0
        };

        let vo = if ocnt > 0 {
          dp[0][zcnt][ocnt - 1] + dp[1][zcnt][ocnt - 1]
        } else {
          0
        } - if ocnt > limit {
          dp[0][zcnt][ocnt - limit - 1]
        } else {
          0
        };

        dp[0][zcnt][ocnt] = vz % 1000000007;
        dp[1][zcnt][ocnt] = vo % 1000000007;
      }
    }

    let ans = ((dp[0][zero][one] + dp[1][zero][one]) % 1000000007) as i32;
    if ans < 0 { ans + 1000000007 } else { ans }
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::number_of_stable_arrays(1, 1, 2));
  println!("{}", Solution::number_of_stable_arrays(1, 2, 1));
  println!("{}", Solution::number_of_stable_arrays(3, 3, 2));
  println!("{}", Solution::number_of_stable_arrays(14, 13, 59));
  println!("{}", Solution::number_of_stable_arrays(1, 3, 1));
  println!("{}", Solution::number_of_stable_arrays(1, 4, 2));
  println!("{}", Solution::number_of_stable_arrays(20, 15, 75));
  println!("{}", Solution::number_of_stable_arrays(31, 36, 60));
  println!("{}", Solution::number_of_stable_arrays(35, 35, 22));
  println!("{}", Solution::number_of_stable_arrays(76, 59, 99));
}
