struct RLEIterator {
  e: Vec<i32>,
}

impl RLEIterator {
  fn new(encoding: Vec<i32>) -> Self {
    RLEIterator {
      e: encoding.clone(),
    }
  }

  fn next(&mut self, n: i32) -> i32 {
    let mut n = n;
    let mut ans = -1;
    while n > 0 && self.e.len() > 0 {
      let diff = self.e[0].min(n);
      self.e[0] -= diff;
      n -= diff;
      ans = self.e[1];
      if n == 0 {
        return ans;
      }
      if self.e.len() > 0 && self.e[0] == 0 {
        self.e.remove(0);
        self.e.remove(0);
      }
    }

    -1
  }
}
