struct Solution {}

impl Solution {
  pub fn trap(height: Vec<i32>) -> i32 {
    let mut stack: Vec<(i32, usize)> = Vec::new();
    let mut sum = 0;
    height.iter().enumerate().for_each(|(idx, v)| {
      if stack.len() == 0 || stack[stack.len() - 1].0 >= *v {
        stack.push((*v, idx));
      } else if stack[stack.len() - 1].0 < *v {
        let mut last = stack.pop().unwrap();
        while stack.len() > 0 && stack[stack.len() - 1].0 < *v {
          let mut prev = stack.pop().unwrap();
          sum += (prev.0 - last.0) * (idx - prev.1 - 1) as i32;
          last = prev;
        }
        if stack.len() > 0 {
          sum += (*v - last.0) * (idx - stack[stack.len() - 1].1 - 1) as i32;
        }

        stack.push((*v, idx));
      }
    });
    sum
  }
}
