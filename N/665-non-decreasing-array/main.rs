struct Solution {}

impl Solution {
  pub fn check_possibility(nums: Vec<i32>) -> bool {
    let mut idx: usize = 0;
    let mut isFirst = true;
    let mut ret = true;
    for i in 1..nums.len() {
      if nums[i] < nums[i - 1] {
        if isFirst {
          isFirst = false;
          idx = i;
        } else {
          ret = false;
          break;
        }
      }
    }

    if ret {
      if (idx > 0 && idx < nums.len() - 1 && nums[idx - 1] > nums[idx + 1])
        && (idx > 1 && nums[idx - 2] > nums[idx])
      {
        ret = false;
      }
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::check_possibility(vec![0; 5]));
}
