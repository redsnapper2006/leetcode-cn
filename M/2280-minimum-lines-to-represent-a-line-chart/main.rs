impl Solution {
  pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
    let mut sp = stock_prices;
    sp.sort_unstable();
    if sp.len() == 1 {
      return 0;
    }
    let mut x: i64 = (sp[1][0] - sp[0][0]) as i64;
    let mut y: i64 = (sp[1][1] - sp[0][1]) as i64;
    let mut cnt: i32 = 1;
    (2..sp.len()).for_each(|idx| {
      let ny = (sp[idx][1] - sp[idx - 1][1]) as i64;
      let nx = (sp[idx][0] - sp[idx - 1][0]) as i64;
      if ny * x != nx * y {
        cnt += 1;
      }
      x = nx;
      y = ny;
    });
    cnt
  }
}
