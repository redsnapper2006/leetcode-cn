struct Solution {}

impl Solution {
  pub fn print_bin(num: f64) -> String {
    let mut ret: Vec<u8> = vec!['0' as u8, '.' as u8];
    let mut n: f64 = num;
    while ret.len() <= 32 && n != 0 as f64 {
      n *= 2 as f64;
      let o = n as i32;
      ret.push(('0' as i32 + o) as u8);
      n -= o as f64;
    }
    if ret.len() <= 32 {
      String::from_utf8(ret).unwrap()
    } else {
      "ERROR".to_string()
    }
  }
}
