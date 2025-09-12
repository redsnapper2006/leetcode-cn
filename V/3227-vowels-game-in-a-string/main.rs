impl Solution {
  pub fn does_alice_win(s: String) -> bool {
    for c in s.chars() {
      if "aeiou".contains(c) {
        return true;
      }
    }
    false
  }

  pub fn does_alice_win2(s: String) -> bool {
    s.chars().any(|x| "aeiou".contains(x))
  }

  pub fn does_alice_win3(s: String) -> bool {
    s.as_bytes()
      .to_vec()
      .iter()
      .filter(|x| **x == b'a' || **x == b'e' || **x == b'i' || **x == b'o' || **x == b'u')
      .collect::<Vec<_>>()
      .len()
      != 0
  }
}
