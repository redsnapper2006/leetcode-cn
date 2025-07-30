struct LUPrefix {
  b: Vec<bool>,
  i: usize,
  n: usize,
}

impl LUPrefix {
  fn new(n: i32) -> Self {
    return LUPrefix {
      b: vec![false; n as usize + 1],
      i: 1,
      n: n as usize,
    };
  }

  fn upload(&mut self, video: i32) {
    self.b[video as usize] = true;
    while self.i <= self.n && self.b[self.i] {
      self.i += 1;
    }
  }

  fn longest(&self) -> i32 {
    self.i as i32 - 1
  }
}
