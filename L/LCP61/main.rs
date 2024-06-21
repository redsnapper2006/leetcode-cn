struct Solution {}

impl Solution {
  pub fn temperature_trend(temperature_a: Vec<i32>, temperature_b: Vec<i32>) -> i32 {
    let zip_ab = temperature_a
      .into_iter()
      .zip(temperature_b.into_iter())
      .collect::<Vec<(i32, i32)>>();

    let mut cnt: i32 = 0;
    let mut ans: i32 = 0;
    (1..zip_ab.len()).for_each(|idx| {
      if zip_ab[idx].0.cmp(&zip_ab[idx - 1].0) == zip_ab[idx].1.cmp(&zip_ab[idx - 1].1) {
        cnt += 1;
        ans = ans.max(cnt);
      } else {
        cnt = 0;
      }
    });
    ans
  }
}
