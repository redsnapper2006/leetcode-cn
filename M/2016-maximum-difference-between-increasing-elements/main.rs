impl Solution {
  pub fn maximum_difference(nums: Vec<i32>) -> i32 {
    let mut min: i32 = i32::MAX;
    let mut ans: i32 = -1;
    for v in &nums {
      min = min.min(*v);
      ans = if *v > min && *v - min > ans {
        v - min
      } else {
        ans
      };
    }
    ans
  }
}
