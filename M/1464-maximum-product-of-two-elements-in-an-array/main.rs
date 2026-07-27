impl Solution {
  pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut mx1: i32 = i32::MIN;
    let mut mx2: i32 = i32::MIN;

    nums.iter().for_each(|&v| {
      if v > mx1 {
        mx2 = mx1;
        mx1 = v;
      } else if v > mx2 {
        mx2 = v;
      }
    });
    (mx1 - 1) * (mx2 - 1)
  }
}
