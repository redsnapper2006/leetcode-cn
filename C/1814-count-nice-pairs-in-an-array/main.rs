struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut ret: i32 = 0;
    let mut ns = nums;
    ns.iter_mut().for_each(|n| {
      let m = *n;
      let mut rev: i32 = 0;
      while *n > 0 {
        rev = rev * 10 + *n % 10;
        *n /= 10;
      }
      let mut v = map.entry(rev - m).or_insert(0);
      ret += *v;
      ret %= 1000000007;
      *v += 1;
    });
    ret % 1000000007
  }
}
