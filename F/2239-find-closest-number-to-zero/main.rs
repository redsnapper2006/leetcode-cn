struct Solution {}

impl Solution {
  pub fn find_closest_number(nums: Vec<i32>) -> i32 {
    let mut abs: i32 = 1000000;
    let mut ans: i32 = 1000000;
    nums.iter().for_each(|&v| {
      if v.abs() < abs {
        abs = v.abs();
        ans = v;
      } else if v.abs() == abs && ans < v {
        ans = v;
      }
    });
    ans
  }
}
