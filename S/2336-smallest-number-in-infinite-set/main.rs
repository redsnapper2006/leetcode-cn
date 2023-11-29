use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct SmallestInfiniteSet {
  buf: BinaryHeap<Reverse<i32>>,
  vis: Vec<bool>,
  idx: i32,
}

impl SmallestInfiniteSet {
  fn new() -> Self {
    SmallestInfiniteSet {
      buf: BinaryHeap::new(),
      vis: vec![false; 1001],
      idx: 0,
    }
  }

  fn pop_smallest(&mut self) -> i32 {
    match self.buf.peek() {
      None => {
        self.idx += 1;
        self.idx
      }
      Some(&Reverse(num)) => {
        self.buf.pop();
        self.vis[num as usize] = false;
        num
      }
    }
  }

  fn add_back(&mut self, num: i32) {
    if self.idx < num || self.vis[num as usize] {
      return;
    } else if self.idx == num {
      self.idx -= 1;
      return;
    } else {
      self.buf.push(Reverse(num));
      self.vis[num as usize] = true;
    }
  }
}
