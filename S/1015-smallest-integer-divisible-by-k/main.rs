struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
    let mut visit: HashSet<i32> = HashSet::new();
    visit.insert(1);

    let mut res: i32 = 1;
    let mut n: i32 = 1;
    while n % k != 0 {
      n = (n * 10 + 1) % k;
      if visit.contains(&n) {
        return -1;
      }
      visit.insert(n);
      res += 1;
    }

    res
  }
}
