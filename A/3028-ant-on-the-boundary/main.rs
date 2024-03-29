impl Solution {
  pub fn return_to_boundary_count(nums: Vec<i32>) -> i32 {
    nums
      .iter()
      .fold((0, 0), |(sum, cnt), x| {
        if sum + x == 0 {
          (0, cnt + 1)
        } else {
          (sum + x, cnt)
        }
      })
      .1
  }
}
