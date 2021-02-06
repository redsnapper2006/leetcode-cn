struct Solution {}

impl Solution {
  pub fn decode_at_index(s: String, kk: i32) -> String {
    let bb = s.as_bytes();
    let mut cnt: u64 = 0;
    for i in 0..bb.len() {
      if bb[i] <= '9' as u8 && bb[i] >= '0' as u8 {
        cnt *= (bb[i] - '0' as u8) as u64;
      } else {
        cnt += 1;
      }
    }
    let mut b: u8 = '0' as u8;
    let mut k: u64 = kk as u64;
    for i in (0..bb.len()).rev() {
      k %= cnt;

      if k == 0 && bb[i as usize] <= 'z' as u8 && bb[i as usize] >= 'a' as u8 {
        b = bb[i as usize];
        break;
      }
      if bb[i as usize] <= '9' as u8 && bb[i as usize] >= '0' as u8 {
        cnt /= (bb[i as usize] - '0' as u8) as u64;
      } else {
        cnt -= 1;
      }
    }
    String::from(b as char)
  }
}

fn main() {
  println!("{}", Solution::decode_at_index(String::from("l2c3"), 4));
}
