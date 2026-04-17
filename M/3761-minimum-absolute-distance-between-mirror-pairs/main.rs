use std::collections::HashMap;

impl Solution {
  fn reverse(n: i32) -> i32 {
    let mut ans: i32 = 0;
    let mut n = n;
    while n > 0 {
      ans = ans * 10 + n % 10;
      n /= 10;
    }
    ans
  }

  pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, usize> = HashMap::new();
    let mut ans: usize = nums.len();
    for i in 0..nums.len() {
      if m.contains_key(&nums[i]) {
        ans = ans.min(i - m.get(&nums[i]).unwrap());
      }
      m.insert(Self::reverse(nums[i]), i);
    }
    if ans == nums.len() { -1 } else { ans as _ }
  }
}
