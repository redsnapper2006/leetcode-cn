impl Solution {
  pub fn judge_circle(moves: String) -> bool {
    moves
      .as_bytes()
      .iter()
      .fold((true, 0, 0), |(ans, mut lr, mut ud), b| {
        lr += match b {
          b'L' => 1,
          b'R' => -1,
          _ => 0,
        };
        ud += match b {
          b'U' => 1,
          b'D' => -1,
          _ => 0,
        };
        (lr == 0 && ud == 0, lr, ud)
      })
      .0
  }
}
