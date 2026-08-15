struct DataStream {
  k: i32,
  v: i32,
  p: i32,
}

impl DataStream {
  fn new(value: i32, k: i32) -> Self {
    DataStream {
      v: value,
      k: k,
      p: k,
    }
  }

  fn consec(&mut self, num: i32) -> bool {
    if self.v == num {
      self.p -= 1;
    } else {
      self.p = self.k;
    }
    self.p <= 0
  }
}
