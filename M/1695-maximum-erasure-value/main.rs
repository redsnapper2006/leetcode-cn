impl Solution {
  pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
    let mut buf: Vec<i32> = vec![0; 10001];
    let mut sum: i32 = 0;
    let mut ans: i32 = 0;

    let mut left: usize = 0;
    let mut right: usize = 0;
    while right < nums.len() {
      let off = nums[right] as usize;
      buf[off] += 1;
      sum += nums[right];
      while buf[off] > 1 {
        buf[nums[left] as usize] -= 1;
        sum -= nums[left];
        left += 1;
      }
      ans = ans.max(sum);
      right += 1;
    }
    ans
  }
}
