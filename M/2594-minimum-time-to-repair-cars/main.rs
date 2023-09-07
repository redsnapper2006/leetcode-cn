struct Solution {}

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct G {
  r: u32,
  c: u32,
}

impl PartialOrd for G {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let sdiff = (self.c * self.c * self.r) as u64;
    let odiff = (other.c * other.c * other.r) as u64;
    sdiff.partial_cmp(&odiff)
  }
}

impl PartialEq for G {
  fn eq(&self, other: &Self) -> bool {
    let sdiff = (self.c * self.c * self.r) as u64;
    let odiff = (other.c * other.c * other.r) as u64;
    sdiff == odiff
  }
}

impl Ord for G {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl Eq for G {}

impl Solution {
  pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let mut bh: BinaryHeap<Reverse<G>> = BinaryHeap::new();

    ranks.iter().for_each(|&v| {
      bh.push(Reverse(G { r: v as u32, c: 1 }));
    });

    let mut res: u64 = 0;
    (0..cars).for_each(|_| {
      let g = bh.pop().unwrap().0;
      // println!("{} {}", g.r, g.c);
      if res < g.r as u64 * g.c as u64 * g.c as u64 {
        res = g.r as u64 * g.c as u64 * g.c as u64;
      }
      // println!("{} ", res);
      bh.push(Reverse(G { r: g.r, c: g.c + 1 }));
    });
    res as i64
  }
}

fn main() {
  println!("{}", Solution::repair_cars(vec![4, 2, 3, 1], 10));
  println!("{}", Solution::repair_cars(vec![5, 1, 8], 6));
}
