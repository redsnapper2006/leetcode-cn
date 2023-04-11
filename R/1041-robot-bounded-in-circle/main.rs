struct Solution {}

impl Solution {
  pub fn is_robot_bounded(instructions: String) -> bool {
    let (x, y, d) = instructions
      .as_bytes()
      .iter()
      .fold((0, 0, 0), |(x, y, d), &b| match b as char {
        'G' => match d {
          0 => (x, y + 1, d),
          1 => (x + 1, y, d),
          2 => (x, y - 1, d),
          3 => (x - 1, y, d),
          _ => (x, y, d),
        },
        'L' => (x, y, (d + 3) % 4),
        'R' => (x, y, (d + 1) % 4),
        _ => (x, y, d),
      });

    d != 0 || (x == 0 && y == 0)
  }
}
