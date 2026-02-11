impl Solution {
  pub fn total_steps(nums: Vec<i32>) -> i32 {
    let mut stack: Vec<(i32, i32)> = vec![];
    let mut ans: i32 = 0;
    for i in 0..nums.len() {
      let mut step: i32 = 0;
      while stack.len() > 0 && stack[stack.len() - 1].0 <= nums[i] {
        step = step.max(stack.pop().unwrap().1);
      }

      if stack.len() == 0 {
        stack.push((nums[i], 0));
      } else {
        stack.push((nums[i], step + 1));
        ans = ans.max(step + 1);
      }
    }

    ans
  }
}
