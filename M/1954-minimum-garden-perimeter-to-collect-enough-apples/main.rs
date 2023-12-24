impl Solution {
  pub fn minimum_perimeter(needed_apples: i64) -> i64 {
    let mut sum: i64 = 0;
    let mut i: i64 = 0;
    while sum < needed_apples {
      i += 1;
      sum += 4 * (i * (i + 1) + i * (2 * i + 1)) - 4 * 2 * i;
    }

    i * 8
  }
}
