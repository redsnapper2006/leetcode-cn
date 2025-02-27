struct TextEditor {
  l: Vec<u8>,
  r: Vec<u8>,
}

impl TextEditor {
  fn new() -> Self {
    TextEditor {
      l: vec![],
      r: vec![],
    }
  }

  fn add_text(&mut self, text: String) {
    text.as_bytes().iter().for_each(|&b| {
      self.l.push(b);
    });
  }

  fn delete_text(&mut self, k: i32) -> i32 {
    let s = self.l.len().min(k as usize);
    self.l.resize(self.l.len() - s, b'-');
    s as _
  }

  fn cursor_left(&mut self, k: i32) -> String {
    let l = self.l.len().min(k as usize);
    (0..l).for_each(|_| {
      self.r.push(self.l.pop().unwrap());
    });
    let r = self.l.len().min(10);
    String::from_utf8(self.l[(self.l.len() - r)..].to_vec()).unwrap()
  }

  fn cursor_right(&mut self, k: i32) -> String {
    let r = self.r.len().min(k as usize);
    (0..r).for_each(|_| {
      self.l.push(self.r.pop().unwrap());
    });
    let l = self.l.len().min(10);
    String::from_utf8(self.l[(self.l.len() - l)..].to_vec()).unwrap()
  }
}
