impl Solution {
  pub fn min_operations(n: i32) -> i32 {
    let mut n = n;
    let mut buf: Vec<i32> = vec![];
    while n > 0 {
      buf.push(n % 2);
      n /= 2;
    }
    let mut idx: usize = 0;
    while idx < buf.len() && buf[idx] == 0 {
      idx += 1;
    }

    let mut ans: i32 = 0;
    let mut cnts: Vec<i32> = vec![];
    let mut cnt: i32 = 0;
    let mut d: i32 = buf[idx];
    while idx < buf.len() {
      if buf[idx] == d {
        cnt += 1;
      } else {
        cnts.push(cnt);
        d = buf[idx];
        cnt = 1;
      }
      idx += 1;
    }
    cnts.push(cnt);

    for i in (0..cnts.len()).step_by(2) {
      if cnts[i] == 1 {
        ans += 1;
      } else {
        if i < cnts.len() - 1 {
          ans += if cnts[i + 1] == 1 { 1 } else { 2 };
          cnts[i + 2] += if cnts[i + 1] == 1 { 1 } else { 0 };
        } else {
          ans += 2;
        }
      }
    }
    ans
  }
}
