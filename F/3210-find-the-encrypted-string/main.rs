impl Solution {
  pub fn get_encrypted_string(s: String, k: i32) -> String {
    let bb = s.as_bytes().to_vec();
    let mut ans: Vec<u8> = Vec::new();

    (0..bb.len()).for_each(|idx| {
      ans.push(bb[(idx + k as usize) % bb.len()]);
    });
    String::from_utf8(ans).unwrap()
  }
}
