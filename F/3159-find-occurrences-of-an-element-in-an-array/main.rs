struct Solution {}

impl Solution {
  pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
    let filter = nums
      .into_iter()
      .enumerate()
      .filter(|v| v.1 == x)
      .collect::<Vec<(usize, i32)>>();
    queries.iter().map(|&q| {
      if q <= filter.len() as i32  {
        filter[q as usize-1].0 as i32
      } else {
        -1
      }
    }).collect::<Vec<i32>>()
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::occurrences_of_element(vec![1, 3, 1, 7], vec![1, 3, 2, 4], 1)
  );
}
