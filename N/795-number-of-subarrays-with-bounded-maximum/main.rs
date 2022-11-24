struct Solution {}

impl Solution {
  pub fn num_subarray_bounded_max(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    let mut last1: i32 = -1;
    let mut last2: i32 = -1;
    let mut ret: i32 = 0;
    for (i, v) in nums.iter().enumerate() {
      if *v >= left && *v <= right {
        last1 = i as i32;
      } else if *v > right {
        last2 = i as i32;
        last1 = -1;
      }
      if last1 != -1 {
        ret += last1 - last2;
      }
    }
    ret
  }

  pub fn num_subarray_bounded_max2(nums: Vec<i32>, left: i32, right: i32) -> i32 {
    let (mut left_idx, mut right_idx): (Vec<i32>, Vec<i32>) =
      (vec![nums.len() as i32; nums.len()], vec![-1; nums.len()]);

    let mut stack: Vec<i32> = Vec::new();
    let mut idx: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
      if stack.len() == 0 || stack[stack.len() - 1] >= nums[i] {
        stack.push(nums[i]);
        idx.push(i as i32);
        continue;
      }
      let mut j: i32 = stack.len() as i32 - 1;
      while j >= 0 && stack[j as usize] < nums[i] {
        left_idx[idx[j as usize] as usize] = i as i32;
        j -= 1;
      }
      stack.resize((j + 1) as usize, -1);
      idx.resize((j + 1) as usize, -1);
      stack.push(nums[i]);
      idx.push(i as i32);
    }
    stack.clear();
    idx.clear();
    for i in (0..nums.len()).rev() {
      if stack.len() == 0 || stack[stack.len() - 1] > nums[i] {
        stack.push(nums[i]);
        idx.push(i as i32);
        continue;
      }
      let mut j: i32 = stack.len() as i32 - 1;
      while j >= 0 && stack[j as usize] <= nums[i] {
        right_idx[idx[j as usize] as usize] = i as i32;
        j -= 1;
      }
      stack.resize((j + 1) as usize, -1);
      idx.resize((j + 1) as usize, -1);
      stack.push(nums[i]);
      idx.push(i as i32);
    }

    let mut ret: i32 = 0;
    for i in 0..nums.len() {
      if nums[i] >= left && nums[i] <= right {
        ret += (i as i32 - right_idx[i]) * (left_idx[i] - i as i32);
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::num_subarray_bounded_max(vec![1, 2, 3, 4], 2, 3)
  );
}
