use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct DinnerPlates {
  c: usize,
  b: Vec<Vec<i32>>,
  l: BinaryHeap<Reverse<usize>>,
  r: BinaryHeap<usize>,
}

impl DinnerPlates {
  fn new(capacity: i32) -> Self {
    DinnerPlates {
      c: capacity as usize,
      b: Vec::new(),
      l: BinaryHeap::new(),
      r: BinaryHeap::new(),
    }
  }

  fn push(&mut self, val: i32) {
    match self.l.peek() {
      None => {
        self.b.push(vec![val]);
        self.r.push(self.b.len() - 1);
        if self.b[self.b.len() - 1].len() < self.c {
          self.l.push(Reverse(self.b.len() - 1));
        }
      }
      Some(&idx) => {
        self.b[idx.0].push(val);
        if self.b[idx.0].len() == self.c {
          self.l.pop();
        }
        if self.b[idx.0].len() == 1 {
          self.r.push(idx.0);
        }
      }
    }
  }

  fn pop(&mut self) -> i32 {
    while matches!(self.r.peek(),  Some(&idx) if  self.b[idx].len() == 0) {
      self.r.pop();
    }

    match self.r.peek() {
      None => -1,
      Some(&idx) => {
        if self.b[idx].len() == self.c {
          self.l.push(Reverse(idx));
        }
        self.b[idx].pop().unwrap()
      }
    }
  }

  fn pop_at_stack(&mut self, index: i32) -> i32 {
    if self.b.len() < index as usize + 1 || self.b[index as usize].len() == 0 {
      return -1;
    }
    if self.b[index as usize].len() == self.c {
      self.l.push(Reverse(index as usize));
    }

    self.b[index as usize].pop().unwrap()
  }
}

fn main() {
  let mut o = DinnerPlates::new(2);
  o.push(1);
  println!("{}", o.pop());
  println!("{}", o.pop_at_stack(3));
}
