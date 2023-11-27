struct Solution {}

impl Solution {
  pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
    let mut stack: Vec<(i32, usize)> = Vec::new();
    let mut left: Vec<usize> = vec![arr.len(); arr.len()];
    let mut right: Vec<usize> = vec![arr.len(); arr.len()];

    arr.iter().enumerate().for_each(|(idx, &v)| {
      while stack.len() > 0 && v <= stack[stack.len() - 1].0 {
        let (t, offset) = stack.pop().unwrap();
        left[offset] = idx;
      }
      stack.push((v, idx));
    });
    stack.clear();
    arr.iter().rev().enumerate().for_each(|(idx, &v)| {
      while stack.len() > 0 && v < stack[stack.len() - 1].0 {
        let (t, offset) = stack.pop().unwrap();
        right[arr.len() - 1 - offset] = idx;
      }
      stack.push((v, idx));
    });

    let mut ret: i64 = 0;
    (0..arr.len()).for_each(|idx| {
      ret +=
        (arr[idx] as i64 * (left[idx] - idx) as i64 * (idx + 1 + right[idx] - arr.len()) as i64)
          % 1000000007;
      ret %= 1000000007;
    });
    ret as i32
  }
}
