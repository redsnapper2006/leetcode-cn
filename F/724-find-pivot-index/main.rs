struct Solution {}

impl Solution {
  pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut left: Vec<i32> = vec![0; nums.len()];
    let mut right: Vec<i32> = vec![0; nums.len()];
    let mut ls: i32 = 0;
    let mut rs: i32 = 0;

    (0..nums.len()).for_each(|idx| {
      left[idx] = ls;
      right[nums.len() - 1 - idx] = rs;
      ls += nums[idx];
      rs += nums[nums.len() - 1 - idx];
    });

    let mut idx: usize = 0;
    while idx < nums.len() {
      if left[idx] == right[idx] {
        return idx as i32;
      }

      idx += 1;
    }
    -1
  }
}

fn main() {
  println!("{}", Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]));
}
