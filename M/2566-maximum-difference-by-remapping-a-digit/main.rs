impl Solution {
  pub fn min_max_difference(num: i32) -> i32 {
    let mut buf: Vec<i32> = Vec::new();
    let mut n = num;
    while n > 0 {
      buf.push(n % 10);
      n /= 10;
    }
    let mut max: i32 = 0;
    let mut is_found = false;
    let mut num: i32 = 0;
    (0..buf.len()).rev().for_each(|i| {
      if buf[i] != 9 {
        if !is_found || buf[i] == num {
          num = buf[i];
          is_found = true;
          max = max * 10 + 9;
        } else {
          max = max * 10 + buf[i];
        }
      } else {
        max = max * 10 + 9;
      }
    });

    let mut min: i32 = 0;
    num = buf[buf.len() - 1];
    (0..buf.len()).rev().for_each(|i| {
      if buf[i] == num {
        min = min * 10 + 0;
      } else {
        min = min * 10 + buf[i];
      }
    });

    max - min
  }
}
