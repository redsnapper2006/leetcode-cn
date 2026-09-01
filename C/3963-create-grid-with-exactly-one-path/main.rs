impl Solution {
  pub fn create_grid(m: i32, n: i32) -> Vec<String> {
    (0..m).fold(vec![], |mut ans, r| {
      match r {
        0 => ans.push(".".repeat(n as usize)),
        _ => ans.push("#".repeat(n as usize - 1) + "."),
      }
      ans
    })
  }
}
