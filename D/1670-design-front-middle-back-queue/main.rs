struct FrontMiddleBackQueue {
  l: Vec<i32>,
  lhe: usize,
  lta: usize,
  r: Vec<i32>,
  rhe: usize,
  rta: usize,
}

impl FrontMiddleBackQueue {
  fn new() -> Self {
    Self {
      l: vec![0; 2010],
      lhe: 1010,
      lta: 1010,
      r: vec![0; 2010],
      rhe: 1010,
      rta: 1010,
    }
  }

  fn push_front(&mut self, val: i32) {
    self.l[self.lhe] = val;
    self.lhe -= 1;
    self.update();
  }

  fn push_middle(&mut self, val: i32) {
    self.lta += 1;
    self.l[self.lta] = val;
    self.update();
  }

  fn push_back(&mut self, val: i32) {
    self.rta += 1;
    self.r[self.rta] = val;
    self.update();
  }

  fn pop_front(&mut self) -> i32 {
    if self.lta == self.lhe && self.rhe == self.rta {
      return -1;
    }
    let mut ret: i32 = 0;
    if self.lta == self.lhe {
      self.rhe += 1;
      ret = self.r[self.rhe];
    } else {
      self.lhe += 1;
      ret = self.l[self.lhe];
    }
    self.update();
    ret
  }

  fn pop_middle(&mut self) -> i32 {
    if self.lta == self.lhe && self.rhe == self.rta {
      return -1;
    }
    let mut ret: i32 = 0;
    if self.lta - self.lhe == self.rta - self.rhe {
      ret = self.l[self.lta];
      self.lta -= 1;
    } else {
      self.rhe += 1;
      ret = self.r[self.rhe];
    }
    self.update();
    ret
  }

  fn pop_back(&mut self) -> i32 {
    if self.lta == self.lhe && self.rhe == self.rta {
      return -1;
    }
    let ret = self.r[self.rta];
    self.rta -= 1;
    self.update();
    ret
  }
  fn update(&mut self) {
    while self.lta - self.lhe > self.rta - self.rhe {
      self.r[self.rhe] = self.l[self.lta];
      self.rhe -= 1;
      self.lta -= 1;
    }
    while self.rta - self.rhe > self.lta - self.lhe + 1 {
      self.rhe += 1;
      self.lta += 1;
      self.l[self.lta] = self.r[self.rhe];
    }
  }
}
