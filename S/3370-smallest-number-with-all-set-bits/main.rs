impl Solution {
  pub fn smallest_number(n: i32) -> i32 {
    let mut ans: i32 = 1;
    while ans <= n {
      ans <<= 1;
    }
    ans - 1
  }
}
