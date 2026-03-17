impl Solution {
  pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<(i32, usize)> = vec![];
    for c in 0..matrix[0].len() {
      buf.push((0, c));
    }

    let mut ans: i32 = 0;
    for r in 0..matrix.len() {
      for c in 0..buf.len() {
        if matrix[r][buf[c].1] == 1 {
          buf[c].0 = if r > 0 { buf[c].0 } else { 0 } + 1;
        } else {
          buf[c].0 = 0;
        }
      }
      buf.sort_unstable();

      for c in 0..buf.len() {
        ans = ans.max(buf[c].0 * (matrix[0].len() - c) as i32);
      }
    }
    ans
  }

  pub fn largest_submatrix2(matrix: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; matrix[0].len()]; matrix.len()];
    let mut max = 0;
    for i in 0..matrix.len() {
      for j in 0..matrix[0].len() {
        let mut p = 0;
        if i > 0 {
          p = buf[i - 1][j];
        }
        if matrix[i][j] == 1 {
          buf[i][j] = p + 1;
        } else {
          buf[i][j] = 0;
        }
      }
      let mut t: Vec<i32> = buf[i].to_vec();
      t.sort();
      t.reverse();
      for i in 0..t.len() {
        // println!("{}", t[i]);
        if t[i] * (i as i32 + 1) > max {
          max = t[i] * (i as i32 + 1);
        }
      }
    }
    max
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::largest_submatrix(vec![vec![0; 5]; 5]));
}
