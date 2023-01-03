struct Solution {}

impl Solution {
  pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut posBuf: Vec<usize> = Vec::new();

    let mut stack: Vec<i32> = Vec::new();
    let mut ret: Vec<i32> = vec![-1; nums.len()];
    for i in 0..nums.len() {
      if stack.len() == 0 || stack[stack.len() - 1] > nums[i] {
        stack.push(nums[i]);
        posBuf.push(i);
      } else {
        let mut offset: i32 = -1;
        for j in (0..stack.len()).rev() {
          if stack[j] < nums[i] {
            ret[posBuf[j]] = nums[i];
          } else {
            offset = j as i32;
            break;
          }
        }
        stack.resize(offset as usize + 1, 0);
        posBuf.resize(offset as usize + 1, 0);
        stack.push(nums[i]);
        posBuf.push(i);
      }
    }

    let mut i: usize = 0;
    let mut j: usize = stack.len() - 1;
    while i < nums.len() && j >= 0 {
      if stack[j] < nums[i] {
        ret[posBuf[j]] = nums[i];
        j -= 1;
      } else {
        i += 1;
      }
    }
    for m in 0..j + 1 {
      ret[posBuf[m]] = -1;
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::next_greater_elements(vec![0; 5]));
}
