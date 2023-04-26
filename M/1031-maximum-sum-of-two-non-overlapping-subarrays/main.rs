struct Solution {}

impl Solution {
  pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
    let mut left: Vec<i32> = vec![0; nums.len()];
    let mut right: Vec<i32> = vec![0; nums.len()];

    let mut sum: i32 = 0;
    let mut r_sum: i32 = 0;
    (0..first_len).for_each(|idx| {
      sum += nums[idx as usize];
      r_sum += nums[nums.len() - 1 - idx as usize];
    });

    let mut max_sum = sum;
    let mut max_r_sum = r_sum;
    (first_len as usize..=nums.len() - second_len as usize).for_each(|idx| {
      left[idx] = max_sum;
      sum -= nums[idx - first_len as usize];
      sum += nums[idx];
      if max_sum < sum {
        max_sum = sum;
      }

      right[nums.len() - idx] = max_r_sum;
      r_sum -= nums[nums.len() - 1 - idx + first_len as usize];
      r_sum += nums[nums.len() - 1 - idx];
      if max_r_sum < r_sum {
        max_r_sum = r_sum;
      }
    });

    let mut ret: i32 = 0;
    let mut second_sum: i32 = 0;
    (0..second_len as usize - 1).for_each(|idx| {
      second_sum += nums[idx];
    });
    (second_len as usize - 1..nums.len()).for_each(|idx| {
      second_sum += nums[idx];
      if idx >= second_len as usize {
        second_sum -= nums[idx - second_len as usize];
      }

      if idx >= (first_len + second_len - 1) as usize
        && second_sum + left[idx - second_len as usize + 1] > ret
      {
        ret = second_sum + left[idx - second_len as usize + 1];
      }
      if idx + (first_len as usize) < nums.len() && second_sum + right[idx + 1] > ret {
        ret = second_sum + right[idx + 1];
      }
    });

    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2)
  );
  println!(
    "{}",
    Solution::max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2)
  );
  println!(
    "{}",
    Solution::max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3)
  );
}
