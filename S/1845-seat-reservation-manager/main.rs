use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct SeatManager {
  n: i32,
  c: i32,
  h: BinaryHeap<Reverse<i32>>,
}

impl SeatManager {
  fn new(n: i32) -> Self {
    SeatManager {
      n: n,
      c: 0,
      h: BinaryHeap::new(),
    }
  }

  fn reserve(&mut self) -> i32 {
    match self.h.len() > 0 {
      true => self.h.pop().unwrap().0,
      _ => {
        self.c += 1;
        self.c
      }
    }
  }

  fn unreserve(&mut self, seat_number: i32) {
    self.h.push(Reverse(seat_number));
  }
}
