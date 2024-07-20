struct Solution {}

impl Solution {
  pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
    let mut more: Vec<(usize, usize, i32)> = Vec::new();
    let mut zeros: Vec<(usize, usize)> = Vec::new();
    (0..3).for_each(|row| {
      (0..3).for_each(|col| {
        if grid[row][col] == 0 {
          zeros.push((row, col));
        } else if grid[row][col] > 1 {
          let mut t = grid[row][col] - 1;
          while t > 0 {
            more.push((row, col, 0));
            t -= 1;
          }
        }
      });
    });

    let mut ans: i32 = 100;

    fn dfs(
      zeros: &Vec<(usize, usize)>,
      zidx: usize,
      more: &mut Vec<(usize, usize, i32)>,
      step: i32,
      ans: &mut i32,
    ) {
      if zidx == zeros.len() {
        if step < *ans {
          *ans = step;
        }
        return;
      }

      let mut midx: usize = 0;
      while midx < more.len() {
        if more[midx].2 == 1 {
          midx += 1;
          continue;
        }
        more[midx].2 = 1;
        dfs(
          zeros,
          zidx + 1,
          more,
          step
            + (zeros[zidx].0 as i32 - more[midx].0 as i32).abs()
            + (zeros[zidx].1 as i32 - more[midx].1 as i32).abs(),
          ans,
        );
        more[midx].2 = 0;
        midx += 1;
      }
    }
    dfs(&zeros, 0, &mut more, 0, &mut ans);

    ans
  }
}

fn main() {
  println!(
    "{}",
    Solution::minimum_moves(vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]])
  );
  println!(
    "{}",
    Solution::minimum_moves(vec![vec![1, 3, 0], vec![1, 0, 0], vec![1, 0, 3]])
  );
  println!(
    "{}",
    Solution::minimum_moves(vec![vec![3, 2, 0], vec![0, 1, 0], vec![0, 3, 0]])
  );
  println!(
    "{}",
    Solution::minimum_moves(vec![vec![1, 2, 2], vec![1, 1, 0], vec![0, 1, 1]])
  );
}
