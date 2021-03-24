struct Solution {}

impl Solution {
  pub fn find132pattern(nums: Vec<i32>) -> bool {
    let mut r: Vec<Vec<i32>> = Vec::new();
    let mut idx: i32 = -1;
    for i in 1..nums.len() {
      if nums[i] > nums[i - 1] {
        idx = i as i32 - 1;
        break;
      }
    }
    if idx == -1 {
      return false;
    }
    let mut start: i32 = nums[idx as usize];
    let mut end: i32 = nums[idx as usize];
    for i in idx as usize + 1..nums.len() {
      for j in 0..r.len() {
        if r[j][0] < nums[i] && r[j][1] > nums[i] {
          return true;
        }
      }

      if nums[i] > end {
        end = nums[i];
      } else if nums[i] < end {
        if nums[i] > start {
          return true;
        }
        r.push(vec![start, end]);
        start = nums[i];
        end = nums[i];
      }
    }
    false
  }
}

fn main() {
  println!("{}", Solution::find132pattern(vec![3, 1, 4, 2]));
  println!("{}", Solution::find132pattern(vec![1, 2, 3, 4]));
  println!("{}", Solution::find132pattern(vec![-1, 3, 2, 0]));
  println!("{}", Solution::find132pattern(vec![1, 0, 1, -4, -3]));
  println!("{}", Solution::find132pattern(vec![3, 1, 4, 2]));
}
