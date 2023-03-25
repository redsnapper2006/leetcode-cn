struct Solutioin {}

impl Solution {
  pub fn maximum_count(nums: Vec<i32>) -> i32 {
    let (p, n): (i32, i32) = nums.iter().fold((0, 0), |(p, n), &v| {
      if v > 0 {
        (p + 1, n)
      } else if v < 0 {
        (p, n + 1)
      } else {
        (p, n)
      }
    });
    p.max(n)
  }
}
