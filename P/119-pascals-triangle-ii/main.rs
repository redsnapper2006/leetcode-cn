struct Solution {}

impl Solution {
  pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![1; row_index as usize + 1];

    (1..row_index).for_each(|idx| {
      (1..=idx).rev().for_each(|i| {
        let ii = i as usize;
        ans[ii] += ans[ii - 1];
      });
    });
    ans
  }
}

fn main() {
  println!("{:?}", Solution::get_row(4));
}
