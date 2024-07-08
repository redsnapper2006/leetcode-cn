struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    let mut start: usize = 0;
    let mut ans: usize = 0;
    nums.iter().enumerate().for_each(|(idx, &v)| {
      m.entry(v).and_modify(|x| *x += 1).or_insert(1);

      while *m.get(&v).unwrap() > k {
        m.entry(nums[start]).and_modify(|x| *x -= 1);
        start += 1;
      }
      ans = ans.max(idx - start + 1);
    });

    ans as i32
  }
}
