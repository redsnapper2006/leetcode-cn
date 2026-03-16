impl Solution {
  pub fn score_difference(nums: Vec<i32>) -> i32 {
    nums
      .iter()
      .enumerate()
      .fold((0, 0), |(ans, offset), (idx, n)| {
        let new_offset =
          (offset + if idx % 6 == 5 { 1 } else { 0 } + if n % 2 == 1 { 1 } else { 0 }) % 2;
        (ans + n * if new_offset == 0 { 1 } else { -1 }, new_offset)
      })
      .0
  }
}
