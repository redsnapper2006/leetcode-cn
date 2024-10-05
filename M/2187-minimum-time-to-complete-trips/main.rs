struct Solution {}

impl Solution {
  pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
    let mut s: i64 = 1;
    let mut e: i64 = 100000000000002;

    while s <= e {
      let m = s + (e - s) / 2;
      let mut sum: i64 = 0;
      time.iter().for_each(|&v| {
        let v = v as i64;
        sum += m / v;
      });
      if sum >= total_trips as i64 {
        e = m - 1;
      } else {
        s = m + 1;
      }
    }
    s
  }
}

fn main() {
  println!(
    "{}",
    Solution::minimum_time(
      vec![
        39, 82, 69, 37, 78, 14, 93, 36, 66, 61, 13, 58, 57, 12, 70, 14, 67, 75, 91, 1, 34, 68, 73,
        50, 13, 40, 81, 21, 79, 12, 35, 18, 71, 43, 5, 50, 37, 16, 15, 6, 61, 7, 87, 43, 27, 62,
        95, 45, 82, 100, 15, 74, 33, 95, 38, 88, 91, 47, 22, 82, 51, 19, 10, 24, 87, 38, 5, 91, 10,
        36, 56, 86, 48, 92, 10, 26, 63, 2, 50, 88, 9, 83, 20, 42, 59, 55, 8, 15, 48, 25
      ],
      4187
    )
  );
}
