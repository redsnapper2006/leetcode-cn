impl Solution {
  pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut less, mut equal) = (0i32, 0i32);
    for n in nums {
      less += if n < target { 1 } else { 0 };
      equal += if n == target { 1 } else { 0 };
    }
    (less..less + equal).collect()
  }

  pub fn target_indices2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();
    let (mut start, mut end) = (-1i32, -1i32);
    for i in 0..nums.len() {
      if nums[i] == target {
        if start == -1 {
          start = i as i32;
        }
        end = i as i32;
      }
    }
    if start == -1 {
      vec![]
    } else {
      (start..=end).collect()
    }
  }
}
