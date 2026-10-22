impl Solution {
  pub fn min_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let Some(z) = nums.iter().position(|&x| x == 0) else {
      return -1;
    };
    if nums[z..]
      .iter()
      .chain(nums[..z].iter())
      .enumerate()
      .all(|(i, &x)| x == i as i32)
    {
      return z.min(n - z + 2) as i32;
    }
    if nums[..=z]
      .iter()
      .rev()
      .chain(nums[z + 1..].iter().rev())
      .enumerate()
      .all(|(i, &x)| x == i as i32)
    {
      return (z + 2).min(n - z) as i32;
    }

    -1
  }
}
