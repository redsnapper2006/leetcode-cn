struct Solution {}

impl Solution {
  pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
    let mut agg: i32 = 0;
    let mut sum: f64 = 0.0;
    let mut idx: usize = 0;
    while agg <= income && idx < brackets.len() {
      let mut v = brackets[idx][0] - agg;
      if v > income - agg {
        v = income - agg;
      }

      sum += v as f64 * brackets[idx][1] as f64 / 100 as f64;
      agg = brackets[idx][0];
      idx += 1;
    }

    sum
  }
}
