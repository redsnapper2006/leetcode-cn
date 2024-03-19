struct Solution {}

impl Solution {
  pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
    let mut stack: Vec<(i32, usize)> = Vec::new();
    let mut left: Vec<usize> = vec![0; nums.len()];
    let mut right: Vec<usize> = vec![0; nums.len()];

    nums.iter().enumerate().for_each(|(idx, &v)| {
      while stack.len() > 0 && stack[stack.len() - 1].0 > v {
        let (p, off) = stack.pop().unwrap();
        left[off] = idx - 1;
      }
      stack.push((v, idx));
    });
    let (v, idx) = stack.pop().unwrap();
    left[idx] = idx;
    let mut idx_base = idx;
    while stack.len() > 0 {
      let (v, idx) = stack.pop().unwrap();
      left[idx] = idx_base;
    }

    stack.clear();
    nums.iter().enumerate().rev().for_each(|(idx, &v)| {
      while stack.len() > 0 && stack[stack.len() - 1].0 > v {
        let (p, off) = stack.pop().unwrap();
        right[off] = idx + 1;
      }
      stack.push((v, idx));
    });
    let (v, idx) = stack.pop().unwrap();
    right[idx] = idx;
    let mut idx_base = idx;
    while stack.len() > 0 {
      let (v, idx) = stack.pop().unwrap();
      right[idx] = idx_base;
    }

    let mut max: i32 = 0;
    (0..nums.len()).for_each(|idx| {
      let r = right[idx] as i32;
      let l = left[idx] as i32;
      if l >= k && r <= k && max < (l - r + 1) * nums[idx] {
        max = (l - r + 1) * nums[idx]
      }
    });
    max
  }
}
