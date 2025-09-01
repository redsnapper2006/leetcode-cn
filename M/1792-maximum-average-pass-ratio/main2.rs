use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct P(f64, f64);

impl P {
  pub fn rate(&self) -> f64 {
    (self.0 + 1.0) / (self.1 + 1.0) - self.0 / self.1
  }
}

impl PartialOrd for P {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.rate().partial_cmp(&other.rate())
  }
}

impl PartialEq for P {
  fn eq(&self, other: &Self) -> bool {
    self.rate() == other.rate()
  }
}

impl Ord for P {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

impl Eq for P {}

impl Solution {
  pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
    let mut h: BinaryHeap<P> = BinaryHeap::new();
    for c in classes {
      h.push(P(c[0] as f64, c[1] as f64));
    }

    (0..extra_students).for_each(|_| {
      let mut t = h.pop().unwrap();
      t.0 += 1.0;
      t.1 += 1.0;
      h.push(t);
    });

    let mut sum: f64 = 0.0;
    let mut cnt: i32 = 0;
    while let Some(P(p, t)) = h.pop() {
      sum += p / t;
      cnt += 1;
    }

    sum / (cnt as f64)
  }
}
