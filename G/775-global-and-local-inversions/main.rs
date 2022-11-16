struct Solution {}

impl Solution {
  pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
    for (i, v) in nums.iter().enumerate() {
      if v - i as i32 > 1 || i as i32 - v > 1 {
        return false;
      }
    }
    true
  }

  pub fn is_ideal_permutation2(nums: Vec<i32>) -> bool {
    let mut local: u32 = 0;
    let mut global: u32 = 0;

    for i in 0..nums.len() - 1 {
      if nums[i] > nums[i + 1] {
        local += 1;
      }
    }

    let mut stack: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
      if stack.len() == 0 || stack[stack.len() - 1] > nums[i] {
        global += stack.len();
        stack.push(nums[i]);
      } else {
        let mut offset: usize = 0;
        for j in (0..stack.len()).rev() {
          if stack[j] > nums[i] {
            offset = j + 1;
            break;
          }
        }

        global += offset;
        stack.insert(offset, nums[i]);
      }
      println!("{:?} {}", stack, global);
    }
    println!("{} {}", local, global);
    local == global
  }
}

fn main() {
  // println!("{}", Solution::is_ideal_permutation(vec![1, 2, 3, 4]));
  // println!("{}", Solution::is_ideal_permutation(vec![1, 0, 2]));
  println!("{}", Solution::is_ideal_permutation(vec![1, 2, 0]));
}
