impl Solution {
  pub fn min_absolute_difference(nums: Vec<i32>, x: i32) -> i32 {
    let x = x as usize;
    let mut buf: Vec<i32> = vec![];

    let mut ans: i32 = i32::MAX;
    for i in (x..nums.len()) {
      let ll = match buf.binary_search(&nums[i - x]) {
        Ok(ov) => ov,
        Err(ev) => ev,
      };
      buf.insert(ll, nums[i - x]);

      let ll = match buf.binary_search(&nums[i]) {
        Ok(ov) => ov,
        Err(ev) => ev,
      };
      if ll > 0 {
        ans = ans.min(nums[i] - buf[ll - 1]);
      }
      if ll < buf.len() {
        ans = ans.min(buf[ll] - nums[i]);
      }
    }
    ans
  }
}
