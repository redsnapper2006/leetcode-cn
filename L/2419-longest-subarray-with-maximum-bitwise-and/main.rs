impl Solution {
  pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut mx: i32 = *nums.iter().max().unwrap();
    let mut cnt: i32 = 0;
    let mut ans: i32 = 0;
    nums.iter().for_each(|&v| {
      if mx == v {
        cnt += 1;
      } else {
        ans = ans.max(cnt);
        cnt = 0;
      }
    });
    ans.max(cnt)
  }
}
