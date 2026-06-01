impl Solution {
  pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
    let mut idx: i32 = cost.len() as i32 - 1;
    cost.sort_unstable();
    let mut sum: i32 = 0;
    while idx >= 0 {
      sum += if idx >= 0 { cost[idx as usize] } else { 0 } + if idx >= 1 { cost[idx as usize - 1] } else { 0 };
      idx -= 3;
    }
    sum
  }
}
