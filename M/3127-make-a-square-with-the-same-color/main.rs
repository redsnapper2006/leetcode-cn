struct Solution {}

impl Solution {
  pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
    (1..3)
      .map(|r| {
        (1..3)
          .map(|c| {
            let v = if grid[r][c] == 'B' { 1 } else { 0 }
              + if grid[r - 1][c] == 'B' { 1 } else { 0 }
              + if grid[r][c - 1] == 'B' { 1 } else { 0 }
              + if grid[r - 1][c - 1] == 'B' { 1 } else { 0 };
            v <= 1 || v >= 3
          })
          .any((|x| x))
      })
      .any(|x| x)
  }
}
