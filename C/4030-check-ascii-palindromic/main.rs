impl Solution {
  pub fn is_palindromic(s: String) -> bool {
    let bb = s.as_bytes().into_iter().map(|b| format!("{b:08b}")).collect::<Vec<String>>();

    let n = bb.len();
    for i in 0..bb.len() / 2 {
      if bb[i].chars().rev().collect::<String>() != bb[n - 1 - i] {
        return false;
      }
    }
    if n % 2 == 1 {
      let mid = bb[n / 2].as_bytes();
      for i in 0..4 {
        if mid[i] != mid[7 - i] {
          return false;
        }
      }
    }

    true
  }
}
