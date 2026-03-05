impl Solution {
  pub fn min_operations(s: String) -> i32 {
    let (mut t0, mut t1): (i32, i32) = (0, 0);
    s.as_bytes().iter().enumerate().for_each(|(idx, b)| {
      if b - b'0' == (idx % 2) as u8 {
        t1 += 1;
      } else {
        t0 += 1;
      }
    });
    t0.min(t1)
  }

  pub fn min_operations2(s: String) -> i32 {
    let buf = s.into_bytes();
    let mut c1: Vec<u8> = vec!['0' as u8; buf.len()];
    let mut c2: Vec<u8> = vec!['0' as u8; buf.len()];

    for i in 0..buf.len() {
      if i % 2 == 0 {
        c1[i] = '1' as u8;
      } else {
        c2[i] = '1' as u8;
      }
    }

    let mut ret1 = 0;
    let mut ret2 = 0;
    for i in 0..buf.len() {
      if buf[i] != c1[i] {
        ret1 += 1;
      }
      if buf[i] != c2[i] {
        ret2 += 1;
      }
    }

    let mut ret = ret1;
    if ret > ret2 {
      ret = ret2;
    }
    ret
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::min_operations(String::from("010")));
}
