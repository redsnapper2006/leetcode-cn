use std::collections::HashMap;

impl Solution {
  pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();
    let mut m: HashMap<i32, (i32, i32)> = HashMap::new();
    nums.iter().for_each(|&v| {
      m.entry(v % space).or_insert((v, 0)).1 += 1;
    });

    let mut ans = i32::MAX;
    let mut cnt = 0i32;
    for (_, &(org, c)) in &m {
      if c > cnt || c == cnt && ans > org {
        cnt = c;
        ans = org;
      }
    }
    ans
  }
}
