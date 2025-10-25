impl Solution {
  pub fn total_money(n: i32) -> i32 {
    let w = n / 7;
    let r = n % 7;
    (w * 7 * (w + 7) + r * (w * 2 + r + 1)) / 2
  }
}
