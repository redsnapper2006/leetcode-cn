struct Solution {}

impl Solution {
  pub fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
    let size = nums.len();
    let mut m = nums;

    (0..m.len() - 1).for_each(|idx| {
      if m[idx] == m[idx + 1] {
        m[idx] *= 2;
        m[idx + 1] = 0;
      }
    });
    let mut mm = m.into_iter().filter(|&v| v != 0).collect::<Vec<i32>>();
    while mm.len() < size {
      mm.push(0);
    }
    mm
  }
}
