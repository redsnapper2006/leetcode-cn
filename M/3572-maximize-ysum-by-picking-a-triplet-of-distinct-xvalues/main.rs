impl Solution {
  pub fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
    let mut ct: Vec<(i32, i32)> = y
      .into_iter()
      .zip(x.into_iter())
      .collect::<Vec<(i32, i32)>>();
    ct.sort_unstable();

    let mut sum: i32 = 0;
    let mut x1: i32 = -1;
    let mut x2: i32 = -1;
    for i in (0..ct.len()).rev() {
      if x1 == -1 {
        x1 = ct[i].1;
        sum += ct[i].0;
      } else if x2 == -1 {
        if ct[i].1 != x1 {
          x2 = ct[i].1;
          sum += ct[i].0;
        }
      } else {
        if ct[i].1 != x1 && ct[i].1 != x2 {
          return sum + ct[i].0;
        }
      }
    }

    -1
  }
}
