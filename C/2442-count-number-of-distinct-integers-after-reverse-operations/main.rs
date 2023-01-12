struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
    let mut set: HashSet<i32> = HashSet::new();
    for n in nums {
      set.insert(n);
      let mut m = n;
      let mut ret: i32 = 0;
      while m > 0 {
        ret = ret * 10 + m % 10;
        m /= 10;
      }
      set.insert(ret);
    }
    set.len() as i32
  }
}
