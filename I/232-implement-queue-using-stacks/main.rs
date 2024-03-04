struct MyQueue {
  ls: Vec<i32>,
  rs: Vec<i32>,
}

impl MyQueue {
  fn new() -> Self {
    MyQueue {
      ls: Vec::new(),
      rs: Vec::new(),
    }
  }

  fn push(&mut self, x: i32) {
    self.ls.push(x);
  }

  fn pop(&mut self) -> i32 {
    if self.rs.len() == 0 {
      while self.ls.len() > 0 {
        self.rs.push(self.ls.pop().unwrap());
      }
    }
    self.rs.pop().unwrap()
  }

  fn peek(&mut self) -> i32 {
    let v = self.pop();
    self.rs.push(v);
    v
  }

  fn empty(&self) -> bool {
    self.ls.len() == 0 && self.rs.len() == 0
  }
}
