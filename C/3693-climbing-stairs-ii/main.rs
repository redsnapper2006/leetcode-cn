impl Solution {
  pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
    let mut prev = vec![0i32, 0, 0];
    for &cost in &costs {
      let cur = (0i32..3i32).fold(i32::MAX, |acc, i| {
        acc.min(prev[i as usize] + (i + 1) * (i + 1))
      }) + cost;
      prev[2] = prev[1];
      prev[1] = prev[0];
      prev[0] = cur;
    }

    prev[0]
  }
}
