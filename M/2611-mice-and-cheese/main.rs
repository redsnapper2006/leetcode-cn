struct Solution {}

impl Solution {
  pub fn mice_and_cheese(reward1: Vec<i32>, reward2: Vec<i32>, k: i32) -> i32 {
    let mut rr_zip = reward1
      .into_iter()
      .zip(reward2.into_iter())
      .collect::<Vec<(i32, i32)>>();
    rr_zip.sort_by(|x, y| (&x.0 - &x.1).cmp((&y.0 - &y.1)));

    let mut res: i32 = 0;
    rr_zip.iter().enumerate().for_each(|idx, (r1, r2)| {
      if idx < k {
        res += r1;
      } else {
        res += r2;
      }
    });
    res
  }
}
