impl Solution {
  pub fn best_tower(towers: Vec<Vec<i32>>, center: Vec<i32>, radius: i32) -> Vec<i32> {
    towers
      .iter()
      .fold((vec![-1, -1], 0), |(ans, q), tower| {
        match ((tower[0] - center[0]).abs() + (tower[1] - center[1]).abs() <= radius, ans[0] == -1) {
          (true, true) => (vec![tower[0], tower[1]], tower[2]),
          (true, false) => {
            if tower[2] > q || tower[2] == q && (tower[0] < ans[0] || tower[0] == ans[0] && tower[1] < ans[1]) {
              (vec![tower[0], tower[1]], tower[2])
            } else {
              (ans, q)
            }
          }
          (_, _) => (ans, q),
        }
      })
      .0
  }
}
