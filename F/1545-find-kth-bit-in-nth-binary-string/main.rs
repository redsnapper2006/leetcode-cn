impl Solution {
  pub fn find_kth_bit(n: i32, k: i32) -> char {
    match (k == 1, k == 1 << (n - 1), k < 1 << (n - 1)) {
      (true, _, _) => '0',
      (_, true, _) => '1',
      (_, _, true) => Self::find_kth_bit(n - 1, k),
      (_, _, _) => {
        if Self::find_kth_bit(n - 1, (1 << n) - k) == '0' {
          '1'
        } else {
          '0'
        }
      }
    }
  }
}
