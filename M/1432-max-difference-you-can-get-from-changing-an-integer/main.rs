impl Solution {
  pub fn max_diff(num: i32) -> i32 {
    let mut num = num;
    let mut buf: Vec<i32> = vec![];
    while num > 0 {
      buf.push(num % 10);
      num /= 10;
    }
    let mut max_d: i32 = -1;
    let mut min_d: i32 = -1;
    for i in (0..buf.len()).rev() {
      if buf[i] != 9 && max_d == -1 {
        max_d = buf[i];
      }
      if (i == buf.len() - 1 && buf[i] != 1 || i != buf.len() - 1 && buf[i] != 0 && buf[i] != 1)
        && min_d == -1
      {
        min_d = buf[i];
      }
    }

    let mut ans: i32 = 0;
    (0..buf.len()).rev().for_each(|idx| {
      let mut max = buf[idx];
      let mut min = buf[idx];
      if max == max_d {
        max = 9;
      }
      if min == min_d {
        if buf[buf.len() - 1] == min_d {
          min = 1;
        } else {
          min = 0;
        }
      }

      ans = ans * 10 + max - min;
    });
    ans
  }
}
