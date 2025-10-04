impl Solution {
  pub fn max_area(height: Vec<i32>) -> i32 {
    let mut s: usize = 0;
    let mut e: usize = height.len() - 1;

    let mut ans: i32 = 0;
    while s < e {
      ans = ans.max(height[s].min(height[e]) * (e - s) as i32);
      if height[s] < height[e] {
        s += 1;
      } else {
        e -= 1;
      }
    }
    ans
  }
}
