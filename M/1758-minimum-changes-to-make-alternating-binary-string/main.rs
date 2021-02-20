struct Solution {}

impl Solution {
  pub fn min_operations(s: String) -> i32 {
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

fn main() {
  println!("{}", Solution::min_operations(String::from("010")));
}
