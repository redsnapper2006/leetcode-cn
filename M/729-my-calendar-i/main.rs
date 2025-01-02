struct Solution {}

use std::collections::HashMap;
struct MyCalendar {
  T: HashMap<i32, bool>,
  L: HashMap<i32, bool>,
}

impl MyCalendar {
  fn new() -> Self {
    MyCalendar {
      T: HashMap::new(),
      L: HashMap::new(),
    }
  }

  fn query(&self, start: i32, end: i32, l: i32, r: i32, idx: i32) -> bool {
    if r < start || end < l {
      return false;
    }

    if self.L.contains_key(&idx) && *self.L.get(&idx).unwrap() {
      return true;
    }

    if start <= l && r <= end {
      return self.T.contains_key(&idx) && *self.T.get(&idx).unwrap();
    }

    let mid = (l + r) >> 1;
    self.query(start, end, l, mid, idx * 2) || self.query(start, end, mid + 1, r, idx * 2 + 1)
  }
  fn update(&mut self, start: i32, end: i32, l: i32, r: i32, idx: i32) {
    if r < start || end < l {
      return;
    }
    if start <= l && r <= end {
      self.T.insert(idx, true);
      self.L.insert(idx, true);
    } else {
      let mid = (l + r) >> 1;
      self.update(start, end, l, mid, idx * 2);
      self.update(start, end, mid + 1, r, idx * 2 + 1);
      self.T.insert(idx, true);
      if self.L.contains_key(&(idx * 2))
        && *self.L.get(&(idx * 2)).unwrap()
        && self.L.contains_key(&(idx * 2 + 1))
        && *self.L.get(&(idx * 2 + 1)).unwrap()
      {
        self.L.insert(idx, true);
      }
    }
  }

  fn book(&mut self, start_time: i32, end_time: i32) -> bool {
    let start = start_time;
    let end = end_time;
    if self.query(start, end - 1, 0, 1000000000, 1) {
      return false;
    }
    self.update(start, end - 1, 0, 1000000000, 1);
    true
  }
}
