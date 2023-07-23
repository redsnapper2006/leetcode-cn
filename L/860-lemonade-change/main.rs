struct Solution {}

impl Solution {
  pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut buf: [i32; 3] = [0, 0, 0];
    for b in bills {
      let change = b - 5;
      if b == 20 {
        if buf[1] > 0 && buf[0] > 0 {
          buf[1] -= 1;
          buf[0] -= 1;
        } else if buf[0] > 2 {
          buf[0] -= 3;
        } else {
          return false;
        }
        buf[2] += 1;
      } else if b == 10 {
        if buf[0] > 0 {
          buf[0] -= 1;
        } else {
          return false;
        }
        buf[1] += 1;
      } else {
        buf[0] += 1;
      }
    }
    true
  }
}
