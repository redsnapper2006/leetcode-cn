impl Solution {
  pub fn max_distance(moves: String) -> i32 {
    let bb = moves.as_bytes().to_vec();

    let (mut ud, mut lr): (i32, i32) = (0, 0);
    let mut extra: i32 = 0;
    bb.iter().for_each(|&b| match b {
      b'U' => ud += 1,
      b'D' => ud -= 1,
      b'L' => lr += 1,
      b'R' => lr -= 1,
      _ => extra += 1,
    });

    ud.abs() + lr.abs() + extra
  }
}
