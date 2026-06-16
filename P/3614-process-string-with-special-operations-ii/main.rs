impl Solution {
  pub fn process_str(s: String, k: i64) -> char {
    let cc: Vec<char> = s.chars().collect();
    let mut cnt: Vec<i64> = vec![];
    let mut c: i64 = 0;
    cc.iter().for_each(|&b| {
      if b == '*' {
        c = (c - 1).max(0);
      } else if b == '#' {
        c *= 2;
      } else if b != '%' {
        c += 1;
      }
      cnt.push(c);
    });

    if c <= k {
      return '.';
    }

    let mut k = k;
    for i in (0..cc.len()).rev() {
      if cc[i] == '#' {
        if k >= cnt[i] / 2 {
          k -= cnt[i] / 2;
        }
      } else if cc[i] == '%' {
        k = cnt[i] - k - 1;
      } else if cc[i] != '*' && k == cnt[i] - 1 {
        return cc[i];
      }
    }
    '.'
  }
}
