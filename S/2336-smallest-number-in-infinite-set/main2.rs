struct SmallestInfiniteSet {
  buf: Vec<i32>,
  r: i32,
}

impl SmallestInfiniteSet {
  fn new() -> Self {
    SmallestInfiniteSet {
      buf: vec![0; 1001],
      r: 1000,
    }
  }

  fn pop_smallest(&mut self) -> i32 {
    let mut idx: usize = 1;
    while idx <= 1000 && self.buf[idx] == 1 {
      idx += 1;
    }
    if idx <= 1000 {
      self.buf[idx] = 1;
      return idx as i32;
    }

    self.r += 1;
    self.r
  }

  fn add_back(&mut self, num: i32) {
    self.buf[num as usize] = 0;
  }
}
