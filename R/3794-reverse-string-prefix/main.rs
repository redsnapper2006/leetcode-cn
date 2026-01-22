impl Solution {
  pub fn reverse_prefix(s: String, k: i32) -> String {
    let k = k as usize;
    let mut bb = s.as_bytes().to_vec();
    for i in 0..k / 2 {
      let t = bb[i];
      bb[i] = bb[k - 1 - i];
      bb[k - 1 - i] = t;
    }
    String::from_utf8(bb).unwrap()
  }
}
