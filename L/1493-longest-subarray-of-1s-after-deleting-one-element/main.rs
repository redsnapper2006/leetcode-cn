impl Solution {
  pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut prev: i32 = -1;
    let mut cur: i32 = 0;
    let mut ans: i32 = 0;
    nums.iter().for_each(|&v| {
      if v == 0 {
        prev = cur;
        cur = 0;
      } else {
        cur += 1;
      }
      ans = ans.max(cur + prev);
    });
    ans
  }
}
