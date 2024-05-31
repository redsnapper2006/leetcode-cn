impl Solution {
  pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len() as i32;
    let total = (1 + n * n) * n * n / 2;
    let square_total = (1..=n * n).fold(0, |aggr, v| aggr + v * v);
    let mut sum: i32 = 0;
    let mut square_sum: i32 = 0;
    grid.iter().for_each(|row| {
      row.iter().for_each(|&val| {
        sum += val;
        square_sum += val * val;
      });
    });

    let d_minus = sum - total;
    let d_plus = (square_sum - square_total) / d_minus;

    vec![(d_plus + d_minus) / 2, (d_plus - d_minus) / 2]
  }

  pub fn find_missing_and_repeated_values2(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let mut buf: Vec<i32> = vec![0; grid.len() * grid.len()];

    grid.iter().for_each(|row| {
      row.iter().for_each(|&val| {
        buf[val as usize - 1] += 1;
      });
    });

    let mut missing: i32 = 0;
    let mut duplicate: i32 = 0;
    buf.iter().enumerate().for_each(|(idx, val)| {
      if *val == 0 {
        missing = idx as i32 + 1;
      } else if *val == 2 {
        duplicate = idx as i32 + 1;
      }
    });
    vec![duplicate, missing]
  }
}
