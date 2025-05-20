struct Solution {}

impl Solution {
  pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut idx: usize = 0;
    let mut start: usize = 0;
    let mut end: i32 = nums.len() as i32 - 1;
    while idx < nums.len() {
      while idx as i32 <= end && nums[idx] == 2 {
        let t = nums[end as usize];
        nums[end as usize] = nums[idx];
        nums[idx] = t;
        end -= 1;
      }
      if nums[idx] == 0 {
        let t = nums[idx];
        nums[idx] = nums[start];
        nums[start] = t;
        start += 1;
      }
      idx += 1;
    }
  }
}

fn main() {
  let mut a = vec![2, 1, 0, 2, 1];
  Solution::sort_colors(&mut a);
  println!("{:?}", a);
}
