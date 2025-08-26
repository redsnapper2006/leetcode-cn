impl Solution {
  pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
    dimensions
      .iter()
      .fold((0, 0), |acc, d| {
        match (
          d[0] * d[0] + d[1] * d[1] > acc.0,
          d[0] * d[0] + d[1] * d[1] == acc.0,
          d[0] * d[1] > acc.1,
        ) {
          (true, _, _) => (d[0] * d[0] + d[1] * d[1], d[0] * d[1]),
          (false, true, true) => (d[0] * d[0] + d[1] * d[1], d[0] * d[1]),
          (_, _, _) => acc,
        }
      })
      .1
  }
}

struct Solution {}
