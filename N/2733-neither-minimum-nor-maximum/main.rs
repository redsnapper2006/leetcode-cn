struct Solution {}

impl Solution {
  pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
    let mut min: i32 = nums[0];
    let mut max: i32 = nums[0];

    let mut idx: usize = 0;
    while idx < nums.len() {
      let v = nums[idx];
      if min > v {
        if min != max {
          return min;
        }
        min = v;
      } else if max < v {
        if max != min {
          return max;
        }
        max = v;
      } else if v < max && v > min {
        return v;
      }
      idx += 1;
    }
    -1
  }
}
