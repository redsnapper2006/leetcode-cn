struct Solution {}

impl Solution {
  pub fn nth_ugly_number(n: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0; n as usize];
    buf[0] = 1;
    let mut i2: usize = 0;
    let mut i3: usize = 0;
    let mut i5: usize = 0;
    for i in 1..n {
      let v2 = buf[i2] * 2;
      let v3 = buf[i3] * 3;
      let v5 = buf[i5] * 5;
      let mut min = v2;
      if min > v3 {
        min = v3;
      }
      if min > v5 {
        min = v5;
      }
      buf[i as usize] = min;
      if min == v2 {
        i2 += 1;
      }
      if min == v3 {
        i3 += 1;
      }
      if min == v5 {
        i5 += 1;
      }
    }
    buf[n as usize - 1]
  }
}

fn main() {
  println!("{}", Solution::nth_ugly_number(1352));
}
