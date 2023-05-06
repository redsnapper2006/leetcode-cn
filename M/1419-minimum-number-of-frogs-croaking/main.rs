struct Solution {}

impl Solution {
  pub fn min_number_of_frogs(croak_of_frogs: String) -> i32 {
    let mut buf: [i32; 5] = [0; 5];
    let mut res: i32 = 0;
    let mut cs = croak_of_frogs.chars();
    while let Some(b) = cs.next() {
      let mut idx: usize = 0;
      let idx = match b {
        'c' => 0,
        'r' => 1,
        'o' => 2,
        'a' => 3,
        'k' => 4,
        _ => 0,
      };
      buf[idx] += 1;
      if idx > 0 && buf[idx] > buf[idx - 1] {
        return -1;
      }
      if idx == 4 {
        if res < buf[0] {
          res = buf[0];
        }

        buf[0] -= buf[4];
        buf[1] -= buf[4];
        buf[2] -= buf[4];
        buf[3] -= buf[4];
        buf[4] -= buf[4];
      }
    }
    match buf.iter().sum() {
      0 => res,
      _ => -1,
    }
  }
}
