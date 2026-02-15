impl Solution {
  pub fn add_binary(a: String, b: String) -> String {
    let mut aa = a.as_bytes().to_vec();
    let mut bb = b.as_bytes().to_vec();
    aa.reverse();
    bb.reverse();
    let mut ans: Vec<u8> = vec![];
    let mut plus: bool = false;
    for i in 0..aa.len().max(bb.len()) {
      let mut v = if i < aa.len() { aa[i] - b'0' } else { 0 }
        + if i < bb.len() { bb[i] - b'0' } else { 0 }
        + if plus { 1 } else { 0 };
      plus = false;
      if v > 1 {
        v %= 2;
        plus = true;
      }
      ans.push(v + b'0');
    }
    if plus {
      ans.push(b'1');
    }
    ans.reverse();
    String::from_utf8(ans).unwrap()
  }
}
