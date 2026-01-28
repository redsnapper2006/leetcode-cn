use std::collections::BinaryHeap;

impl Solution {
  pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    let mut heap: BinaryHeap<(i32, i32, usize, usize)> = BinaryHeap::new();

    fn build(
      grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, heap: &mut BinaryHeap<(i32, i32, usize, usize)>,
    ) {
      dp[0][0] = 0;
      heap.push((grid[0][0], 0, 0, 0));
      for r in 0..grid.len() {
        for c in 0..grid[0].len() {
          if r == 0 && c == 0 {
            continue;
          }
          dp[r][c] = dp[r][c].min(
            grid[r][c]
              + if r > 0 { dp[r - 1][c] } else { i32::MAX }.min(if c > 0 {
                dp[r][c - 1]
              } else {
                i32::MAX
              }),
          );
          heap.push((grid[r][c], -dp[r][c], r, c));
        }
      }
    }
    build(&grid, &mut dp, &mut heap);

    (0..k).for_each(|_| {
      let mut minCost: i32 = i32::MAX;
      while heap.len() > 0 {
        let (g, cost, r, c) = heap.pop().unwrap();
        let cost = -cost;
        minCost = minCost.min(cost);

        dp[r][c] = dp[r][c].min(minCost);
      }

      build(&grid, &mut dp, &mut heap);
    });

    dp[grid.len() - 1][grid[0].len() - 1]
  }
}
