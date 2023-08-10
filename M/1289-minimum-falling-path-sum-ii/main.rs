struct Solution {}

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct G {
  U: usize,
  I: i32,
}

impl PartialOrd for G {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.I.partial_cmp(&other.I)
  }
}

impl PartialEq for G {
  fn eq(&self, other: &Self) -> bool {
    self.I == other.I
  }
}

impl Ord for G {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl Eq for G {}

impl Solution {
  pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    if grid.len() == 1 {
      return grid[0][0];
    }

    let mut buf: BinaryHeap<Reverse<G>> = BinaryHeap::new();
    (0..grid.len()).for_each(|idx| {
      buf.push(Reverse(G { U: idx, I: 0 }));
    });
    grid.iter().for_each(|row| {
      let g1 = buf.pop().unwrap();
      let g2 = buf.pop().unwrap();

      let mut t: BinaryHeap<Reverse<G>> = BinaryHeap::new();
      row.iter().enumerate().for_each(|(idx, v)| {
        if idx == g1.0.U {
          t.push(Reverse(G {
            U: idx,
            I: v + g2.0.I,
          }));
        } else {
          t.push(Reverse(G {
            U: idx,
            I: v + g1.0.I,
          }));
        }
      });
      buf = t;
    });

    buf.pop().unwrap().0.I
  }
}

fn main() {
  println!(
    "{}",
    Solution::min_falling_path_sum(vec![vec![1, 2, 1], vec![4, 5, 6], vec![7, 8, 9]])
  );
}
