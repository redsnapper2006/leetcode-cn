impl Solution {
  pub fn count_triples(n: i32) -> i32 {
    let mut ans: i32 = 0;
    for c in (2..=n).rev() {
      for a in (1..c) {
        let b = ((c * c - a * a) as f64).sqrt() as i32;
        if a * a + b * b == c * c {
          ans += 1;
        }
      }
    }
    ans
  }
}
