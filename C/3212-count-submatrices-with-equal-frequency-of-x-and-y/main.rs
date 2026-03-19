impl Solution {
  pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
    let mut cols: Vec<Vec<i32>> = vec![vec![0; 2]; grid[0].len()];
    let mut ans: i32 = 0;
    for r in 0..grid.len() {
      let mut sum: (i32, i32) = (0, 0);
      for c in 0..grid[0].len() {
        if grid[r][c] != '.' {
          cols[c][if grid[r][c] == 'X' { 0 } else { 1 }] += 1;
        }
        sum.0 += cols[c][0];
        sum.1 += cols[c][1];
        if sum.0 == sum.1 && sum.0 > 0 {
          ans += 1;
        }
      }
    }
    ans
  }
}
