struct Solution {}

impl Solution {
  pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
    let mut idx1: usize = 0;
    let mut loop1: i32 = 0;
    let mut idx2: usize = 0;
    let mut loop2: i32 = 0;
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    while loop1 < n1 {
      if b1[idx1] == b2[idx2] {
        idx2 += 1;
        if idx2 == b2.len() {
          idx2 = 0;
          loop2 += 1;
        }
      }
      idx1 += 1;
      if idx1 == b1.len() {
        idx1 = 0;
        loop1 += 1;
      }
    }
    loop2 / n2
  }
}
