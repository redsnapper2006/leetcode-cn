impl Solution {
  pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() < 2 || grid[0].len() < 2 {
      return 0;
    }

    let mut ans: i32 = 0;
    for r in 0..grid.len() - 2 {
      for c in 0..grid[0].len() - 2 {
        if grid[r + 1][c + 1] != 5 {
          continue;
        }

        let (mask, row0, row1, col0, col1) =
          (0..9).fold((0, 0, 0, 0, 0), |(mask, row0, row1, col0, col1), idx| {
            let v = grid[r + idx / 3][c + idx % 3];
            (
              mask | 1 << v,
              row0 + if idx / 3 == 0 { v } else { 0 },
              row1 + if idx / 3 == 1 { v } else { 0 },
              col0 + if idx % 3 == 0 { v } else { 0 },
              col1 + if idx % 3 == 1 { v } else { 0 },
            )
          });
        if mask == 0b1111111110 && row0 == row1 && col0 == col1 && row0 == 15 && col0 == 15 {
          ans += 1;
        }
      }
    }

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::num_magic_squares_inside(vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]])
  );
}
