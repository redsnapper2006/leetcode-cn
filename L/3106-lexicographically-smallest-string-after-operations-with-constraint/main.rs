struct Solution {}

impl Solution {
  pub fn get_smallest_string(s: String, k: i32) -> String {
    let mut buf: Vec<u8> = s.as_bytes().to_vec();

    let mut idx: usize = 0;
    let mut k = k;
    while idx < buf.len() && k > 0 {
      let diff = (buf[idx] - b'a') as i32;
      let diff2 = 26 - diff;
      if diff > diff2 && diff2 <= k {
        buf[idx] = b'a';
        k -= diff2;
      } else {
        buf[idx] -= diff.min(k) as u8;
        k -= diff.min(k);
      }
      // println!("{}", String::from_utf8(buf.clone()).unwrap());
      idx += 1;
    }
    String::from_utf8(buf).unwrap()
  }
}

fn main() {
  println!("{}", Solution::get_smallest_string("zbbz".to_string(), 3));
  println!("{}", Solution::get_smallest_string("xaxcd".to_string(), 4));
  println!("{}", Solution::get_smallest_string("lol".to_string(), 0));
}
