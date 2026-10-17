use std::collections::HashMap;

impl Solution {
  pub fn min_swaps(nums: Vec<i32>) -> i32 {
    let mut origin: Vec<(i32, i32)> = vec![];
    let mut pos: HashMap<i32, usize> = HashMap::new();
    let mut buf: Vec<(i32, i32)> = vec![];
    for i in 0..nums.len() {
      let mut n = nums[i];
      let mut sum = 0i32;
      while n > 0 {
        sum += n % 10;
        n /= 10;
      }
      origin.push((sum, nums[i]));
      buf.push((sum, nums[i]));
      pos.insert(nums[i], i);
    }
    buf.sort_unstable();

    let mut ans = 0i32;
    for i in 0..origin.len() {
      if origin[i].1 == buf[i].1 {
        continue;
      }
      let p = *pos.get(&buf[i].1).unwrap();
      pos.insert(origin[i].1, p);
      origin[p] = origin[i];
      ans += 1;
    }
    ans
  }
}
