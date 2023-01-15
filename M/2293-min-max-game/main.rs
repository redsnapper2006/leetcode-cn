impl Solution {
  pub fn min_max_game(nums: Vec<i32>) -> i32 {
    let mut n = nums;
    let mut size: usize = n.len() / 2;

    while size >= 1 {
      (0..size).for_each(|idx| {
        n[idx] = match idx % 2 {
          0 => n[2 * idx].min(n[2 * idx + 1]),
          1 => n[2 * idx].max(n[2 * idx + 1]),
          _ => 0,
        }
      });
      size /= 2
    }
    n[0]
  }
}
