
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
      let mut stack : Vec<i32> = Vec::new();
      nums.iter().enumerate().for_each(|(idx, &v) | {
        while stack.len() > 0 && stack[stack.len() - 1] > v && (stack.len() + nums.len() - idx as usize) > k as usize {
          stack.pop();
        }
        stack.push(v);
      });
      stack.truncate(k as usize);
      stack
    }
}
