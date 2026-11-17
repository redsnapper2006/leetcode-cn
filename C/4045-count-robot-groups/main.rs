impl Solution {
  pub fn count_groups(position: Vec<i32>, speed: Vec<i32>, distance: i32) -> i32 {
    let mut ans = 1i32;
    let n = speed.len();
    let mut last_speed = speed[n - 1];
    let mut last_distance = position[n - 1];
    for i in (0..speed.len()).rev() {
      if last_speed >= speed[i] && last_distance > position[i] + distance {
        ans += 1;
        last_speed = speed[i];
      }
      last_distance = position[i];
    }
    ans
  }
}
