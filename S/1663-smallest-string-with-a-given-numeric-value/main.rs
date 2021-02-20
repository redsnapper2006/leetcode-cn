struct Solution {}

impl Solution {
  pub fn get_smallest_string(n: i32, k: i32) -> String {
    let mut cnt = n;
    let mut kk = k;
    let mut buf: Vec<u8> = Vec::new();
    while cnt > 0 {
      if (cnt - 1) * 26 + 1 >= kk {
        buf.push('a' as u8);
        kk -= 1;
      } else {
        let c = kk - (cnt - 1) * 26;
        buf.push(('a' as i32 + c - 1) as u8);
        kk -= c;
      }
      cnt -= 1;
    }
    String::from_utf8(buf).unwrap()
  }
}

fn main() {
  println!("{}", Solution::get_smallest_string(0, 0));
}
