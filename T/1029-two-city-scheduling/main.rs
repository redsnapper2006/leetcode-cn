struct Solution {}

impl Solution {
  pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
    let mut a_sum = costs.iter().fold(0, |acc, v| acc + v[0]);
    let mut diff: Vec<i32> = costs.iter().map(|v| v[1] - v[0]).collect();
    diff.sort();
    diff.resize(costs.len() / 2, 0);

    a_sum + diff.iter().fold(0, |acc, x| acc + x)
  }

  pub fn two_city_sched_cost2(costs: Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 1000000;
    let n: i32 = (costs.len() / 2) as i32;

    fn dfs(
      costs: &Vec<Vec<i32>>,
      sum1: i32,
      sum2: i32,
      sum: &mut i32,
      idx: i32,
      n1: i32,
      n2: i32,
      n: i32,
    ) {
      if n1 == n && n2 == n {
        if sum1 + sum2 < *sum {
          *sum = sum1 + sum2;
        }
        return;
      }
      if n1 < n {
        dfs(
          costs,
          sum1 + costs[idx as usize][0],
          sum2,
          sum,
          idx + 1,
          n1 + 1,
          n2,
          n,
        );
      }
      if n2 < n {
        dfs(
          costs,
          sum1,
          sum2 + costs[idx as usize][1],
          sum,
          idx + 1,
          n1,
          n2 + 1,
          n,
        );
      }
    }
    dfs(&costs, 0, 0, &mut sum, 0, 0, 0, n);
    sum
  }
}
