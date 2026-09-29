impl Solution {
  pub fn min_queen_moves(source: Vec<i32>, target: Vec<i32>) -> i32 {
    if source[0] == target[0] && source[1] == target[1] {
      0
    } else if (source[0] - target[0]).abs() == (source[1] - target[1]).abs()
      || source[0] == target[0]
      || source[1] == target[1]
    {
      1
    } else {
      2
    }
  }
}
