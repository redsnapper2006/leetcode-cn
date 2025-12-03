use std::collections::HashMap;

impl Solution {
  pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    let mut y_cnt: HashMap<i32, i64> = HashMap::new();
    points.iter().for_each(|p| {
      y_cnt.entry(p[1]).and_modify(|x| *x += 1).or_insert(1);
    });

    let mut sum_y: Vec<i64> = vec![];
    let mut sum: i64 = 0;
    for (_, c) in &y_cnt {
      sum += (c * (c - 1) / 2) % 1000000007;
      sum_y.push(sum);
    }

    let mut prev: i64 = 0;
    let mut ans: i64 = 0;
    (0..sum_y.len()).for_each(|i| {
      let bb = (sum_y[i] - prev) % 1000000007;
      let tt = (sum - sum_y[i]) % 1000000007;
      ans += bb * tt;
      ans %= 1000000007;
      prev = sum_y[i];
    });
    ans as _
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::count_trapezoids(vec![
      vec![1, 0],
      vec![2, 0],
      vec![3, 0],
      vec![2, 2],
      vec![3, 2]
    ])
  );
  println!(
    "{}",
    Solution::count_trapezoids(vec![vec![1, 0], vec![0, 0], vec![0, 1], vec![2, 1]])
  )
}
