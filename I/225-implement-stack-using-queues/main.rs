struct MyStack {
  q: Vec<i32>,
  c: usize,
}

impl MyStack {
  fn new() -> Self {
    MyStack {
      q: Vec::new(),
      c: 0,
    }
  }

  fn push(&mut self, x: i32) {
    if self.c >= self.q.len() {
      self.q.push(x);
      self.c = self.q.len();
    } else {
      self.q[self.c] = x;
      self.c+=1;
    }
  }

  fn pop(&mut self) -> i32 {
    self.c -= 1;
    self.q[self.c]
  }

  fn top(&self) -> i32 {
    self.q[self.c - 1]
  }

  fn empty(&self) -> bool {
    self.c == 0
  }
}

fn main() {
  let mut s: MyStack = MyStack::new();
  s.push(1);
  println!("{}", s.pop());
  s.push(2);
  println!("{}", s.top());
}
