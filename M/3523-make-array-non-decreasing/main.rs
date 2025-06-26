impl Solution {
  pub fn maximum_possible_size(nums: Vec<i32>) -> i32 {
    let mut stack  : Vec<i32>  = vec![];

    nums.iter().for_each(|&v | {
      if stack.len() == 0 || stack[stack.len()-1] <= v {
        stack.push(v);
      }
    });
    stack.len() as _
  }
}
