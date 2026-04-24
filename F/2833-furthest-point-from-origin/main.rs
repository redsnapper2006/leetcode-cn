impl Solution {
  pub fn furthest_distance_from_origin(moves: String) -> i32 {
    let (sum, spaces) = moves.as_bytes().iter().fold((0 as i32, 0), |(sum, spaces), &b| {
      (
        sum
          + if b == b'L' {
            1
          } else if b == b'R' {
            -1
          } else {
            0
          },
        spaces + if b == b'_' { 1 } else { 0 },
      )
    });
    sum.abs() + spaces
  }
}
