impl Solution {
  pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
    let (xor, has_non_zero) =
      nums.iter().copied().fold((0, false), |(xor, has), v| (xor ^ v, has | (v != 0)));
    let n = nums.len() as i32;
    match (xor, has_non_zero) {
      (0, false) => 0,
      (0, true) => n - 1,
      _ => n,
    }
  }
}
