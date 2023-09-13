struct Solution {}

impl Solution {
  pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
    if grid[0][0] != 0 {
      return false;
    }
    let mut pos: Vec<(i32, i32)> = vec![(0, 0); grid.len() * grid.len()];

    (0..grid.len()).for_each(|r| {
      (0..grid.len()).for_each(|c| {
        pos[grid[r][c] as usize] = (r as i32, c as i32);
      });
    });

    let mut num: usize = 1;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    while num < grid.len() * grid.len() {
      let (nr, nc) = pos[num];
      let rd = (nr - r).abs();
      let cd = (nc - c).abs();

      if (rd == 1 && cd == 2) || (rd == 2 && cd == 1) {
        r = nr;
        c = nc;
      } else {
        return false;
      }
      num += 1;
    }
    true
  }
}
