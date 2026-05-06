impl Solution {
  pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut ans: Vec<Vec<char>> = vec![vec!['.'; box_grid.len()]; box_grid[0].len()];

    (0..box_grid.len()).for_each(|r| {
      let mut k = box_grid[0].len() - 1;
      (0..box_grid[0].len()).rev().for_each(|c| {
        if box_grid[r][c] == '*' {
          ans[c][box_grid.len() - 1 - r] = '*';
          k = c - 1;
        } else if box_grid[r][c] == '#' {
          ans[k][box_grid.len() - 1 - r] = '#';
          k -= 1;
        }
      });
    });
    ans
  }
}
