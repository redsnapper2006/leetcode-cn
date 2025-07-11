impl Solution {
  pub fn count_rectangles(rectangles: Vec<Vec<i32>>, points: Vec<Vec<i32>>) -> Vec<i32> {
    let mut rect = rectangles;
    rect.sort_unstable();
    let mut dp: Vec<Vec<i32>> = vec![];
    let mut x: Vec<i32> = vec![];
    let mut y: Vec<i32> = vec![0; 101];
    let mut base: i32 = rect[rect.len() - 1][0];
    for i in (0..rect.len()).rev() {
      if rect[i][0] == base {
        y[rect[i][1] as usize] += 1;
      } else {
        for j in (0..100).rev() {
          y[j] += y[j + 1];
        }
        if dp.len() > 0 {
          for j in (0..101) {
            y[j] += dp[dp.len() - 1][j];
          }
        }
        dp.push(y);
        x.push(base);

        base = rect[i][0];
        y = vec![0; 101];
        y[rect[i][1] as usize] = 1;
      }
    }

    for j in (0..100).rev() {
      y[j] += y[j + 1];
    }
    if dp.len() > 0 {
      for j in (0..101) {
        y[j] += dp[dp.len() - 1][j];
      }
    }
    dp.push(y);
    x.push(base);

    let mut ans: Vec<i32> = vec![];
    for p in points {
      let mut off = x.partition_point(|v| v >= &p[0]);
      if off == 0 {
        ans.push(0);
        continue;
      }

      off -= 1;
      ans.push(dp[off][p[1] as usize]);
    }
    ans
  }
}
