impl Solution {
  pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
    let mut buf: Vec<i32> = vec![];
    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        buf.push(grid[r][c]);
      }
    }
    buf.sort_unstable();
    let mut s: usize = 0;
    let mut e: usize = buf.len() - 1;
    let mut ans: i32 = 0;
    while s < e {
      let diff = buf[e] - buf[s];
      if diff % x != 0 {
        return -1;
      }
      ans += diff / x;
      s += 1;
      e -= 1;
    }
    if s == e && (buf[s] - buf[0]) % x != 0 { -1 } else { ans }
  }
}
