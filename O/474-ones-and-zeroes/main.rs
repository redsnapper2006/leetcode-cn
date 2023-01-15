struct Solution {}

impl Solution {
  pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; n as usize + 1]; m as usize + 1];
    buf[0][0] = 1;

    for ss in strs {
      let mut ones: i32 = 0;
      let mut zeros: i32 = 0;
      for b in ss.as_bytes() {
        if *b == '1' as u8 {
          ones += 1;
        } else {
          zeros += 1;
        }
      }
      (0..=m).rev().for_each(|mi| {
        (0..=n).rev().for_each(|ni| {
          if mi - zeros >= 0
            && ni - ones >= 0
            && buf[(mi - zeros) as usize][(ni - ones) as usize] > 0
          {
            buf[mi as usize][ni as usize] = buf[mi as usize][ni as usize]
              .max(buf[(mi - zeros) as usize][(ni - ones) as usize] + 1);
          }
        });
      });
    }

    buf
      .iter()
      .map(|v| v.iter().fold(-1, |acc, &e| e.max(acc)))
      .max()
      .unwrap()
      - 1
  }
}
