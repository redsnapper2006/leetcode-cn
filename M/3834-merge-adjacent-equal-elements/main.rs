impl Solution {
  pub fn merge_adjacent(nums: Vec<i32>) -> Vec<i64> {
    let mut stack: Vec<i64> = Vec::with_capacity(nums.len());
    for n in &nums {
      let mut nn = *n as i64;
      while stack.len() > 0 && *stack.last().unwrap() == nn {
        stack.pop();
        nn <<= 1;
      }
      stack.push(nn);
    }
    stack
  }
}
