struct Solution {}

impl Solution {
  pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut idx: usize = 0;
    let mut ret: i32 = 0;
    while idx < nums.len() - 1 {
      let mut start: usize = idx + 1;
      let mut end: usize = nums.len() - 1;
      while start <= end {
        let m: usize = start + (end - start) / 2;

        if nums[m] + nums[idx] >= target {
          end = m - 1;
        } else {
          start = m + 1;
        }
      }

      ret += (start - idx) as i32 - 1;
      idx += 1;
    }

    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::count_pairs(vec![-5, 0, -7, -1, 9, 8, -9, 9], -14)
  );
}
