impl Solution {
  pub fn max_sum(grid: Vec<Vec<i32>>, limits: Vec<i32>, k: i32) -> i64 {
    let mut grid = grid;
    let mut idx: usize = 0;
    let mut buf: Vec<i32> = vec![];
    while idx < grid.len() {
      grid[idx].sort_by(|x, y| y.cmp(x));

      let mut c: i32 = 0;
      while c < limits[idx] {
        buf.push(grid[idx][c as usize]);
        c += 1;
      }
      idx += 1;
    }
    buf.sort_by(|x, y| y.cmp(x));
    let mut ans: i64 = 0;
    (0..k as usize).for_each(|idx| {
      ans += buf[idx] as i64;
    });
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::max_sum(vec![vec![1, 2], vec![3, 4]], vec![1, 2], 2)
  );
}
