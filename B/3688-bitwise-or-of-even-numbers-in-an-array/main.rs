impl Solution {
  pub fn even_number_bitwise_o_rs(nums: Vec<i32>) -> i32 {
    nums
      .iter()
      .fold(0, |or, &v| or | if v & 1 == 0 { v } else { 0 })
  }
}
