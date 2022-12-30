struct Solution {}

impl Solution {
  pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<i32>> = grid.clone();
    for b in buf.iter_mut() {
      b.sort();
    }

    (0..buf[0].len())
      .map(|idx| buf.iter().map(|r| r[idx]).max().unwrap())
      .sum()
  }

  pub fn delete_greatest_value2(grid: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<Vec<i32>> = grid.clone();
    for b in buf.iter_mut() {
      b.sort();
    }

    let mut ret: i32 = 0;
    for j in 0..buf[0].len() {
      let mut b: Vec<i32> = Vec::new();
      for i in 0..buf.len() {
        b.push(buf[i][j]);
      }
      b.sort();
      ret += b[b.len() - 1];
    }
    ret
  }
}
