struct Solution {}

impl Solution {
  pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    let mut m = nums.clone();
    m.sort();
    m.dedup();
    m.into_iter()
      .filter(|x| *x != 0)
      .collect::<Vec<i32>>()
      .len() as i32
  }
}
