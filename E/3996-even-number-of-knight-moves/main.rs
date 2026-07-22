impl Solution {
  pub fn can_reach(start: Vec<i32>, target: Vec<i32>) -> bool {
    (target[0] + start[0] + target[1] + start[1]) % 2 == 0
  }
}
