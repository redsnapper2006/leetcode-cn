struct Solution {}

impl Solution {
  pub fn find_integers(n: i32) -> i32 {
    let mut buf: [i32; 32] = [0; 32];
    buf[0] = 1;
    buf[1] = 2;
    (2..32).for_each(|idx| {
      buf[idx] = buf[idx - 1] + buf[idx - 2];
    });
    let mut ans: i32 = 0;
    let mut i: i32 = 31;
    while i >= 0 {
      if n & (1 << i) > 0 {
        ans += buf[i as usize];
        let mut j = i - 1;
        if j >= 0 && n & (1 << j) > 0 {
          ans += buf[j as usize] - 1;
          break;
        }
      }

      i -= 1;
    }
    ans + 1
  }
}
