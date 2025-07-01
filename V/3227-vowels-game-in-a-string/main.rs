impl Solution {
  pub fn does_alice_win(s: String) -> bool {
    s.as_bytes()
      .to_vec()
      .iter()
      .filter(|x| **x == b'a' || **x == b'e' || **x == b'i' || **x == b'o' || **x == b'u')
      .collect::<Vec<_>>()
      .len()
      != 0
  }
}
