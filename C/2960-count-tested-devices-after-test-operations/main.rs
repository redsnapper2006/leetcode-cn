impl Solution {
  pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
    let mut cnt: i32 = 0;

    (0..battery_percentages.len()).for_each(|i| {
      if battery_percentages[i] - cnt <= 0 {
        return;
      }
      cnt += 1;
    });
    cnt
  }
}
