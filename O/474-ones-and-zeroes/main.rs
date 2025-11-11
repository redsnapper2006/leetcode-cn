struct Solution {}

impl Solution {
  pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let m = m as usize;
    let n = n as usize;
    let mut buf: Vec<Vec<i32>> = vec![vec![0; n + 1]; m + 1];
    buf[0][0] = 1;

    for ss in strs {
      let mut ones: usize = 0;
      let mut zeros: usize = 0;
      for b in ss.as_bytes() {
        ones += if *b == '1' as u8 { 1 } else { 0 };
        zeros += if *b == '1' as u8 { 0 } else { 1 };
      }
      (0..=m).rev().for_each(|mi| {
        (0..=n).rev().for_each(|ni| {
          if mi >= zeros && ni >= ones && buf[mi - zeros][ni - ones] > 0 {
            buf[mi][ni] = buf[mi][ni].max(buf[mi - zeros][ni - ones] + 1);
          }
        });
      });
    }

    buf.iter().map(|v| v.iter().fold(-1, |acc, &e| e.max(acc))).max().unwrap() - 1
  }
}
