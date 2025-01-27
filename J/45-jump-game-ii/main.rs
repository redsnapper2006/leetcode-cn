struct Solution {}

impl Solution {
  pub fn jump(nums: Vec<i32>) -> i32 {
    let size = nums.len() as i32;
    let mut maxPos: i32 = 0;
    let mut ans: i32 = 0;
    let mut end: i32 = 0;
    (0..nums.len() - 1).for_each(|idx| {
      maxPos = maxPos.max(nums[idx] + idx as i32);
      if idx as i32 == end {
        end = maxPos;
        ans += 1;
      }
    });

    ans
  }
}
