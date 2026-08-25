impl Solution {
  pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
    let mut occupy = 0u128;
    for &n in &nums {
      occupy |= (1 << n);
    }
    let mut b = k;
    while b < 101 && occupy & (1 << b) != 0 {
      b += k;
    }
    b as _
  }
}
