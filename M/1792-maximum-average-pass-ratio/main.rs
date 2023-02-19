use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub struct AVG {
  P: i32,
  T: i32,
}

impl AVG {
  pub fn rate(&self) -> f64 {
    (self.P + 1) as f64 / (self.T + 1) as f64 - (self.P) as f64 / (self.T) as f64
  }
}

impl PartialOrd for AVG {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.rate().partial_cmp(&other.rate())
  }
}

impl PartialEq for AVG {
  fn eq(&self, other: &Self) -> bool {
    self.rate() == other.rate()
  }
}

impl Ord for AVG {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl Eq for AVG {}

impl Solution {
  pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    let mut h: BinaryHeap<AVG> = BinaryHeap::new();
    for c in classes {
      h.push(AVG { P: c[0], T: c[1] });
    }

    (0..extra_students).for_each(|_| {
      let mut t = h.pop().unwrap();
      t.T += 1;
      t.P += 1;
      h.push(t);
    });

    let mut sum: f64 = 0.0;
    let mut cnt: i32 = 0;
    while let Some(avg) = h.pop() {
      sum += (avg.P) as f64 / avg.T as f64;
      cnt += 1;
    }

    sum / (cnt as f64)
  }
}
