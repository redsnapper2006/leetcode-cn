struct Solution {}

impl Solution {
  pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let mut ans: i32 = -1;
    let mut sum: i32 = 0;
    let mut total: i32 = 0;
    (0..gas.len()).for_each(|idx| {
      let d = gas[idx] - cost[idx];
      sum += d;
      total += d;
      if sum < 0 {
        ans = -1;
        sum = 0;
      } else if ans == -1 {
        ans = idx as i32;
      }
    });
    match total < 0 {
      true => -1,
      _ => ans,
    }
  }
}
