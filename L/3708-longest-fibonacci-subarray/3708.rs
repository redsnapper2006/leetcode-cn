impl Solution {
  pub fn longest_subarray(nums: Vec<i32>) -> i32 {
    let mut ans: i32 = 0;
    let mut cnt: i32 = 0;
    for idx in 0..nums.len() - 2 {
      if nums[idx] + nums[idx + 1] == nums[idx + 2] {
        cnt += 1;
      } else {
        ans = ans.max(cnt + 2);
        cnt = 0;
      }
    }

    ans.max(cnt + 2)
  }
}
