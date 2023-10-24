struct Solution {}

impl Solution {
  pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; target as usize + 1]; n as usize + 1];
    (1..=k).for_each(|i| {
      if i > target {
        return;
      }
      buf[1][i as usize] = 1;
    });
    (2..=n).for_each(|i| {
      (i - 1..=target).for_each(|ni| {
        (1..=k).for_each(|v| {
          if ni + v > target {
            return;
          }
          buf[i as usize][ni as usize + v as usize] += buf[i as usize - 1][ni as usize];
          buf[i as usize][ni as usize + v as usize] %= 1000000007;
        });
      });
    });
    buf[n as usize][target as usize]
  }
}

fn main() {
  println!("{}", Solution::num_rolls_to_target(2, 6, 7));
  println!("{}", Solution::num_rolls_to_target(1, 6, 3));
  println!("{}", Solution::num_rolls_to_target(30, 30, 500));
}
