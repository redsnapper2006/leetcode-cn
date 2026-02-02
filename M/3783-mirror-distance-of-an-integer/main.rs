impl Solution {
  pub fn mirror_distance(n: i32) -> i32 {
    let mut nn = n;
    let mut newn = 0;
    while nn > 0 {
      newn = newn * 10 + nn % 10;
      nn /= 10;
    }
    (newn - n).abs()
  }
}
