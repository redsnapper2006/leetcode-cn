use std::collections::VecDeque;
impl Solution {
  pub fn final_string(s: String) -> String {
    let mut buf: VecDeque<char> = VecDeque::new();

    let mut tail: bool = false;
    s.chars().for_each(|b| {
      if b == 'i' {
        tail = !tail;
      } else {
        if tail {
          buf.push_back(b);
        } else {
          buf.push_front(b);
        }
      }
    });

    if tail {
      buf.iter().collect()
    } else {
      buf.iter().rev().collect()
    }
  }
}
