impl Solution {
  pub fn min_absolute_difference(nums: Vec<i32>) -> i32 {
    let (mut p1, mut p2): (i32, i32) = (-1, -1);
    let mut ans: i32 = i32::MAX;
    for i in 0..nums.len() {
      match nums[i] {
        1 => p1 = i as i32,
        2 => p2 = i as i32,
        _ => (),
      };
      if p1 != -1 && p2 != -1 {
        ans = ans.min((p1 - p2).abs());
      }
    }
    match ans {
      i32::MAX => -1,
      _ => ans,
    }
  }
}
