struct Solution {}

impl Solution {
  pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
    let mut left: Vec<i32> = vec![0; nums.len()];
    let mut right: Vec<i32> = vec![0; nums.len()];

    let mut leftMax: i32 = nums[0];
    let mut rightMin: i32 = nums[nums.len() - 1];
    for i in (0..nums.len()) {
      if nums[i] > leftMax {
        leftMax = nums[i]
      }
      if nums[nums.len() - 1 - i] < rightMin {
        rightMin = nums[nums.len() - 1 - i]
      }
      left[i] = leftMax;
      right[nums.len() - 1 - i] = rightMin;
    }

    let mut ret: i32 = -1;
    for i in (0..nums.len() - 1) {
      if left[i] <= right[i + 1] {
        ret = (i + 1) as i32;
        break;
      }
    }
    ret
  }
}
