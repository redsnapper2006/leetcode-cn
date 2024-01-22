struct Solution {}

impl Solution {
  pub fn maximum_swap(num: i32) -> i32 {
    if num < 10 {
      return num;
    }

    let mut buf: Vec<i32> = Vec::new();
    let mut num2 = num;
    while num2 > 0 {
      buf.push(num2 % 10);
      num2 /= 10;
    }
    let mut sort = buf.clone();
    sort.sort();
    let mut idx: usize = buf.len() - 1;
    while idx > 0 {
      if buf[idx] != sort[idx] {
        break;
      }
      idx -= 1;
    }
    if idx == 0 {
      return num;
    }

    let mut idx2: usize = 0;
    while idx2 < buf.len() {
      if buf[idx2] == sort[idx] {
        break;
      }
      idx2 += 1;
    }
    let t = buf[idx2];
    buf[idx2] = buf[idx];
    buf[idx] = t;
    buf.reverse();
    let mut ret = 0;
    idx = 0;
    while idx < buf.len() {
      ret = ret * 10 + buf[idx];
      idx += 1;
    }
    ret
  }
}
