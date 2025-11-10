impl Solution {
  pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut stack: Vec<i32> = vec![];

    let mut ans: i32 = 0;
    nums.iter().for_each(|&v| {
      while stack.len() > 0 && v < stack[stack.len() - 1] {
        ans += 1;
        stack.pop();
      }
      if stack.len() == 0 || v > stack[stack.len() - 1] {
        stack.push(v);
      }
    });
    if stack[0] == 0 {
      ans -= 1;
    }
    ans + stack.len() as i32
  }
}
