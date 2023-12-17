struct Solution {}

impl Solution {
  pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
    let mut buf: Vec<i32> = vec![i32::MAX; cost.len() + 1];

    (0..=1).for_each(|start| {
      let mut idx: usize = start;
      buf[idx] = 0;
      while idx < cost.len() {
        buf[idx + 1] = buf[idx + 1].min(buf[idx] + cost[idx]);
        if idx + 2 <= cost.len() {
          buf[idx + 2] = buf[idx + 2].min(buf[idx] + cost[idx]);
        }
        idx += 1;
      }
    });

    buf[cost.len()]
  }
}
