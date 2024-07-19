struct Solution {}

impl Solution {
  pub fn minimum_levels(possible: Vec<i32>) -> i32 {
    let possible = possible
      .iter()
      .map(|&v| if v == 0 { -1 } else { 1 })
      .collect::<Vec<i32>>();
    let total = possible.iter().sum::<i32>();

    let mut idx: usize = 0;
    let mut sum: i32 = 0;
    while idx < possible.len() {
      sum += possible[idx];
      if sum > total - sum {
        break;
      }
      idx += 1;
    }
    match idx >= possible.len() - 1 {
      true => -1,
      _ => idx as i32 + 1,
    }
  }
}

fn main() {
  println!("{}", Solution::minimum_levels(vec![1, 1, 1, 0]));
}
