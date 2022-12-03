struct Solution {}

impl Solution {
  pub fn second_highest(s: String) -> i32 {
    let bb = s.as_bytes();
    let mut max1: i32 = -1;
    let mut max2: i32 = -1;

    for i in 0..bb.len() {
      if bb[i] >= '0' as u8 && bb[i] <= '9' as u8 {
        let v: i32 = (bb[i] - '0' as u8) as i32;
        if max1 == -1 {
          max1 = v;
        } else {
          if v > max1 {
            max2 = max1;
            max1 = v;
          } else if v < max1 && v > max2 {
            max2 = v;
          }
        }
      }
    }
    max2
  }
}

fn main() {
  println!("{}")
}
