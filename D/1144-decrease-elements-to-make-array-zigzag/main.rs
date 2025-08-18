impl Solution {
  pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
    let mut odd: i32 = 0;
    for i in (1..nums.len()).step_by(2) {
      let v = nums[i - 1].min(if i + 1 >= nums.len() {
        i32::MAX
      } else {
        nums[i + 1]
      });
      odd += if v <= nums[i] { nums[i] - v + 1 } else { 0 };
    }

    let mut even: i32 = 0;
    for i in (0..nums.len()).step_by(2) {
      let v = (if i > 1 { nums[i - 1] } else { i32::MAX }).min(if i < nums.len() - 1 {
        nums[i + 1]
      } else {
        i32::MAX
      });
      even += if v <= nums[i] { nums[i] - v + 1 } else { 0 }
    }
    even.min(odd)
  }
}
