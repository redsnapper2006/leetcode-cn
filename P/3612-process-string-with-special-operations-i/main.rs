use std::collections::VecDeque;

impl Solution {
  pub fn process_str(s: String) -> String {
    let bb = s.as_bytes().to_vec();
    let mut q: VecDeque<u8> = VecDeque::new();

    let mut direction: bool = true;
    bb.iter().for_each(|b| match (b, direction) {
      (b'*', true) => {
        q.pop_back();
      }
      (b'*', false) => {
        q.pop_front();
      }
      (b'#', true) => {
        let buf = q.clone().into_iter().collect::<Vec<u8>>();
        for a in buf {
          q.push_back(a);
        }
      }
      (b'#', false) => {
        let buf = q.clone().into_iter().collect::<Vec<u8>>();
        for a in buf.iter().rev() {
          q.push_front(*a);
        }
      }
      (b'%', _) => {
        direction = !direction;
      }
      (_, true) => {
        q.push_back(*b);
      }
      (_, false) => {
        q.push_front(*b);
      }
    });

    let mut buf: Vec<u8> = vec![];
    if direction {
      for i in q.iter() {
        buf.push(*i);
      }
    } else {
      for i in q.iter().rev() {
        buf.push(*i);
      }
    }

    String::from_utf8(buf).unwrap()
  }
}
