struct Solution {}


impl Solution {
  pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
    let mut left: Vec<i32> = vec![nums.len() as i32; nums.len()];
    let mut right: Vec<i32> = vec![nums.len() as i32; nums.len()];

    let mut stack: Vec<i32> = vec![nums[0]];
    let mut idx: i32 = 1;
    while idx < nums.len() as i32 - 1 {
      let offset = match stack.binary_search(&nums[idx as usize]) {
        Ok(off) => off,
        Err(off) => off,
      };
      if offset == stack.len() {
        stack.push(nums[idx as usize]);
      } else {
        stack[offset] = nums[idx as usize];
      }
      left[idx as usize] = offset as i32 + 1;
      idx += 1;
    }

    stack.clear();
    stack.push(nums[nums.len() - 1]);
    idx = nums.len() as i32 - 2;
    while idx > 0 {
      let offset = match stack.binary_search(&nums[idx as usize]) {
        Ok(off) => off,
        Err(off) => off,
      };
      if offset == stack.len() {
        stack.push(nums[idx as usize]);
      } else {
        stack[offset] = nums[idx as usize];
      }
      right[idx as usize] = offset as i32  +1;
      idx -= 1;
    }

    let mut max: i32 = 0;
    (1..nums.len() - 1).for_each(|i| {
        if left[i] >= 2 && right[i] >=2 {
            max = std::cmp::max(max, left[i] + right[i]-1);
        }

    });
    nums.len() as i32 - max
  }
}
