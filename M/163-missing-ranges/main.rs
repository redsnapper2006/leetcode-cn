struct Solution {}

impl Solution {
  pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
    if nums.len() == 0 {
      return vec![vec![lower, upper]];
    }
    let mut res: Vec<Vec<i32>> = Vec::new();

    let mut start: i32 = lower;
    let mut idx: usize = 0;
    while idx < nums.len() && nums[idx] < lower {
      idx += 1;
    }
    while idx < nums.len() {
      if nums[idx] - 1 >= start {
        res.push(vec![start, nums[idx] - 1]);
      }
      start = nums[idx] + 1;
      idx += 1;
    }
    if upper >= start {
      res.push(vec![start, upper]);
    }

    res
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::find_missing_ranges(vec![1000000000], 0, 1000000000)
  );
}
