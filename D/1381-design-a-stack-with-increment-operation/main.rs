struct CustomStack {
  s: Vec<i32>,
  i: Vec<i32>,
  o: i32,
  m: i32,
}

impl CustomStack {
  fn new(maxSize: i32) -> Self {
    let mx = maxSize as usize;
    CustomStack {
      s: vec![0; mx],
      i: vec![0; mx],
      o: -1,
      m: maxSize,
    }
  }

  fn push(&mut self, x: i32) {
    if self.o == self.m - 1 {
      return;
    }
    self.o += 1;
    self.s[self.o as usize] = x;
  }

  fn pop(&mut self) -> i32 {
    if self.o == -1 {
      return -1;
    }
    let v = self.s[self.o as usize] + self.i[self.o as usize];
    if self.o > 0 {
      self.i[self.o as usize - 1] += self.i[self.o as usize];
    }
    self.i[self.o as usize] = 0;
    self.o -= 1;
    v
  }

  fn increment(&mut self, k: i32, val: i32) {
    let k = k.min(self.o + 1);
    if k > 0 {
      self.i[k as usize - 1] += val;
    }
  }
}
