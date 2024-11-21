impl Solution {
  pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();

    (1..height.len()).for_each(|idx| {
      if height[idx - 1] > threshold {
        ans.push(idx as i32);
      }
    });
    ans
  }
}
