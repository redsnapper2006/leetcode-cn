impl Solution {
  pub fn total_numbers(digits: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    let mut set: HashSet<i32> = HashSet::new();
    let (mut i1, mut i2, mut i3): (usize, usize, usize) = (0, 0, 0);
    while i1 < digits.len() {
      if digits[i1] > 0 {
        i2 = 0;
        while i2 < digits.len() {
          if i1 != i2 {
            i3 = 0;
            while i3 < digits.len() {
              if i1 != i3 && i2 != i3 && digits[i3] % 2 == 0 {
                set.insert(digits[i1] * 100 + digits[i2] * 10 + digits[i3]);
              }
              i3 += 1;
            }
          }
          i2 += 1;
        }
      }
      i1 += 1;
    }
    set.len() as i32
  }
}
