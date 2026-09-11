impl Solution {
  pub fn total_numbers(digits: Vec<i32>) -> i32 {
    let mut buf: [u8; 10] = [0; 10];
    digits.iter().for_each(|&d| {
      buf[d as usize] += 1;
    });

    let mut ans: i32 = 0;
    for i in (0..10).step_by(2) {
      if buf[i] == 0 {
        continue;
      }
      let mut cnt: i32 = 0;
      for j in 0..10 {
        let base = if i == j { 1 } else { 0 };
        cnt += if buf[j] > base { 1 } else { 0 };
        if j != 0 && buf[j] > base + 1 {
          ans += 1;
        }
      }
      ans += (cnt - 1) * (cnt - 1);

      if (i != 0 && buf[0] == 0 || i == 0 && buf[0] == 1) {
        ans += cnt - 1;
      }
    }
    ans
  }

  pub fn total_numbers2(digits: Vec<i32>) -> i32 {
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
