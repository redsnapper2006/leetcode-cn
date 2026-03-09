impl Solution {
  pub fn minimum_index(capacity: Vec<i32>, item_size: i32) -> i32 {
    capacity
      .iter()
      .enumerate()
      .fold((-1, i32::MAX), |(i, s), (idx, &c)| {
        if c >= item_size && c < s {
          (idx as i32, c)
        } else {
          (i, s)
        }
      })
      .0
  }
}
