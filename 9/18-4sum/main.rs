struct Solution {}

impl Solution {
  fn three_sum(nums: Vec<i32>, target: i64) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = Vec::new();
    nums.iter().enumerate().for_each(|(i, &n)| {
      if i > 0 && n == nums[i - 1] {
        return;
      }
      let mut start: usize = i + 1;
      let mut end: usize = nums.len() - 1;

      while start < end {
        if start > i + 1 && start < nums.len() && nums[start] == nums[start - 1] {
          start += 1;
          continue;
        }
        if end < nums.len() - 1 && end >= start && nums[end] == nums[end + 1] {
          end -= 1;
          continue;
        }
        if (nums[i] as i64 + nums[start] as i64 + nums[end] as i64) > target {
          end -= 1;
        } else if (nums[i] as i64 + nums[start] as i64 + nums[end] as i64) < target {
          start += 1;
        } else {
          ret.push(vec![nums[i], nums[start], nums[end]]);
          start += 1;
          end -= 1;
        }
      }
    });
    ret
  }

  pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort();

    let mut ret: Vec<Vec<i32>> = Vec::new();
    nums.iter().enumerate().for_each(|(i, &n)| {
      if i > 0 && n == nums[i - 1] {
        return;
      }

      let r = Self::three_sum(nums[i + 1..].to_vec(), target as i64 - n as i64);

      r.iter().for_each(|v| {
        ret.push(vec![n, v[0], v[1], v[2]]);
      });
    });
    ret
  }
}
