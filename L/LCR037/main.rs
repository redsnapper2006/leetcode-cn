impl Solution {
  pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut idx: usize = 0;
    while idx < asteroids.len() {
      if asteroids[idx] > 0 {
        stack.push(asteroids[idx]);
      } else if stack.len() == 0 || stack[stack.len() - 1] < 0 {
        stack.push(asteroids[idx]);
      } else {
        let mut s_idx: usize = stack.len() - 1;
        while stack.len() > 0
          && stack[stack.len() - 1] > 0
          && stack[stack.len() - 1] < asteroids[idx].abs()
        {
          stack.pop();
        }
        if stack.len() > 0
          && stack[stack.len() - 1] > 0
          && stack[stack.len() - 1] == asteroids[idx].abs()
        {
          stack.pop();
        } else if stack.len() == 0 || stack[stack.len() - 1] < 0 {
          stack.push(asteroids[idx]);
        }
      }
      idx += 1;
    }
    stack
  }
}
