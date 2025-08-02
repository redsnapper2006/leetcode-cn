impl Solution {
  pub fn value_after_k_seconds(n: i32, k: i32) -> i32 {
    let n = n as usize;
    let mut buf: Vec<i32> = vec![1; n];
    for i in 1..=k {
      for j in 1..n {
        buf[j] += buf[j - 1];
        buf[j] %= 1000000007;
      }
    }
    buf[n - 1]
  }
}
