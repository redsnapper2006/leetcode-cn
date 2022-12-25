struct Solution {}

impl Solution {
  pub fn minimum_boxes(n: i32) -> i32 {
    let mut i: i32 = 1;
    let mut nn: i32 = n;
    let mut cur: i32 = 1;
    while nn > cur {
      nn -= cur;
      i += 1;
      cur += i;
    }
    cur = 1;
    let mut j: i32 = 1;
    while nn > cur {
      nn -= cur;
      j += 1;
      cur += 1;
    }
    i * (i - 1) / 2 + j
  }
}
