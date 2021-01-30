struct Solution {}

impl Solution {
  pub fn min_falling_path_sum(a: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; a[0].len()]; a.len()];
    for i in 0..a[0].len() {
      buf[0][i] = a[0][i];
    }
    for i in 1..a.len() {
      for j in 0..a[0].len() {
        let mut v = buf[i - 1][j];
        if j > 0 && v > buf[i - 1][j - 1] {
          v = buf[i - 1][j - 1];
        }
        if j < a[0].len() - 1 && v > buf[i - 1][j + 1] {
          v = buf[i - 1][j + 1];
        }
        buf[i][j] = v + a[i][j];
      }
    }

    let mut max: i32 = 200 * a.len() as i32;
    for i in 0..a[0].len() {
      if max > buf[(a.len() - 1) as usize][i as usize] {
        max = buf[a.len() - 1][i];
      }
    }
    max
  }
}

fn main() {
  println!("{}", Solution::min_falling_path_sum(vec![vec![0; 5]; 5]));
}
