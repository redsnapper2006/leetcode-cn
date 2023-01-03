struct Solution {}

impl Solution {
  pub fn recur(b: &mut Vec<Vec<i32>>, r: usize, c: usize, n: &i32) {
    if b[r][c] != 0 {
      return;
    }
    b[r][c] = *n;
    let cord: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
    for candi in &cord {
      let nr = r as i32 + candi[0];
      let nc = c as i32 + candi[1];
      if nr >= 0 && nr < b.len() as i32 && nc >= 0 && nc < b[0].len() as i32 {
        Solution::recur(b, nr as usize, nc as usize, n);
      }
    }
  }
  pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<i32>> = Vec::new();
    for i in 0..grid.len() {
      buf.push(grid[i].clone());
    }
    for i in 0..grid.len() {
      Solution::recur(&mut buf, i, 0, &1);
      Solution::recur(&mut buf, i, grid[0].len() - 1, &1);
    }
    for i in 0..grid[0].len() {
      Solution::recur(&mut buf, 0, i, &1);
      Solution::recur(&mut buf, grid.len() - 1, i, &1);
    }

    let mut count = 2;
    for i in 0..buf.len() {
      for j in 0..buf[0].len() {
        if buf[i][j] == 0 {
          Solution::recur(&mut buf, i, j, &count);
          count += 1;
        }
      }
    }
    count - 2
  }
}

fn main() {
  println!("{}", Solution::closed_island(vec![vec![0; 5]; 5]));
}
