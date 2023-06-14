struct Solution {}

impl Solution {
  pub fn num_times_all_blue(flips: Vec<i32>) -> i32 {
    let mut max: i32 = 0;

    let mut cnt: i32 = 0;
    flips.iter().enumerate().for_each(|(idx, &v)| {
      if v > max {
        max = v;
      }
      if idx as i32 + 1 == max {
        cnt += 1;
      }
    });
    cnt
  }
}
