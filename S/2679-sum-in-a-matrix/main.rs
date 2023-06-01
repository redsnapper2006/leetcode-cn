struct Solution {}

impl Solution {
  pub fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
    let mut nn: Vec<Vec<i32>> = nums;

    nn.iter_mut().for_each(|row| row.sort());
    (0..nn[0].len()).fold(0, |sum, col| {
      sum + (0..nn.len()).fold(0, |max, row| max.max(nn[row][col]))
    })
  }
}

fn main() {
  println!(
    "{}",
    Solution::matrix_sum(vec![
      vec![7, 2, 1],
      vec![6, 4, 2],
      vec![6, 5, 3],
      vec![3, 2, 1]
    ])
  );
}
