impl Solution {
  pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let div = time / (n - 1);
    let m = time % (n - 1);
    match div % 2 {
      0 => m + 1,
      _ => n - m,
    }
  }
}
