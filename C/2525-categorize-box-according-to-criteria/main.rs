struct Solution {}

impl Solution {
  pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    match (
      length >= 10000
        || width >= 10000
        || height >= 10000
        || length as u64 * width as u64 * height as u64 >= 1000000000 as u64,
      mass >= 100,
    ) {
      (true, true) => "Both".to_string(),
      (true, false) => "Bulky".to_string(),
      (false, true) => "Heavy".to_string(),
      (_, _) => "Neither".to_string(),
    }
  }
}
