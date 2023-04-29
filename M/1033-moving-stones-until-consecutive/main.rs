struct Solution {}

impl Solution {
  pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
    let mut s: Vec<i32> = vec![a, b, c];
    s.sort();

    vec![
      match (s[1] - s[0], s[2] - s[1]) {
        (1, 1) => 0,
        (1, _) => 1,
        (_, 1) => 1,
        (2, _) => 1,
        (_, 2) => 1,
        (_, _) => 2,
      },
      s[2] - s[0] - 2,
    ]
  }
}
