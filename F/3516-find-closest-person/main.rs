impl Solution {
  pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    match (x - z).abs().cmp(&(y - z).abs()) {
      std::cmp::Ordering::Greater => 2,
      std::cmp::Ordering::Less => 1,
      _ => 0,
    }
  }
}
