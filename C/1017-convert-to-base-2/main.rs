struct Solution {}

impl Solution {
  pub fn base_neg2(n: i32) -> String {
    if n == 0 || n == 1 {
      return n.to_string();
    }

    let mut buf: Vec<u8> = Vec::new();
    let mut m = n;
    while m != 0 {
      let r = m & 1;
      buf.push(r as u8 + '0' as u8);
      m -= r;
      m /= -2;
    }
    buf.reverse();

    String::from_utf8(buf).unwrap()
  }
}

fn main() {
  println!("{}", Solution::base_neg2(1));
}
