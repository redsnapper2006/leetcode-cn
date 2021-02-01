struct Solution {}

impl Solution {
  pub fn check_if_can_break(s1: String, s2: String) -> bool {
    let mut buf1: [i32; 26] = [0; 26];
    let mut buf2: [i32; 26] = [0; 26];
    let b1 = s1.as_bytes();
    for i in 0..b1.len() {
      buf1[(b1[i] - 'a' as u8) as usize] += 1;
    }
    let b2 = s2.as_bytes();
    for i in 0..b2.len() {
      buf2[(b2[i] - 'a' as u8) as usize] += 1;
    }

    let mut idx1: usize = 0;
    let mut idx2: usize = 0;
    let mut isOne = true;
    let mut isFirst = true;
    let mut ret = true;
    while idx1 < 26 && idx2 < 26 {
      if buf1[idx1] == 0 {
        idx1 += 1;
        continue;
      }
      if buf2[idx2] == 0 {
        idx2 += 1;
        continue;
      }
      println!("{} {} {} {}", idx1, idx2, buf1[idx1], buf2[idx2]);
      if idx1 == idx2 {
        let mut t = buf1[idx1];
        if t > buf2[idx2] {
          t = buf2[idx2];
        }
        buf1[idx1] -= t;
        buf2[idx2] -= t;
      } else {
        if isFirst {
          isFirst = false;
          if idx1 > idx2 {
            isOne = true;
          } else {
            isOne = false;
          }
        }

        if isOne {
          if idx1 < idx2 {
            ret = false;
            break;
          }
        } else {
          if idx2 < idx1 {
            ret = false;
            break;
          }
        }
        let mut t = buf1[idx1];
        if t > buf2[idx2] {
          t = buf2[idx2];
        }
        buf1[idx1] -= t;
        buf2[idx2] -= t;
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::check_if_can_break(String::from("abc"), String::from("ayx"))
  );
}
