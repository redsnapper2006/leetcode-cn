use std::collections::HashMap;

struct Allocator {
  b: Vec<(i32, i32)>,
  m: HashMap<i32, Vec<(i32, i32)>>,
}

impl Allocator {
  fn new(n: i32) -> Self {
    Allocator {
      b: vec![(0, n - 1)],
      m: HashMap::new(),
    }
  }

  fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
    let mut idx: usize = 0;
    while idx < self.b.len() {
      if self.b[idx].1 - self.b[idx].0 + 1 >= size {
        break;
      }
      idx += 1;
    }
    if idx == self.b.len() {
      return -1;
    }

    let ans = self.b[idx].0;
    if self.b[idx].1 - self.b[idx].0 + 1 == size {
      self.b.remove(idx);
    } else {
      self.b[idx].0 += size;
    }
    self
      .m
      .entry(m_id)
      .and_modify(|x| x.push((ans, ans + size - 1)))
      .or_insert(vec![(ans, ans + size - 1)]);
    ans
  }

  fn free_memory(&mut self, m_id: i32) -> i32 {
    if !self.m.contains_key(&m_id) {
      return 0;
    }

    let mut ans: i32 = 0;

    for occ in self.m.get(&m_id).unwrap() {
      let s = occ.0;
      let e = occ.1;
      let ii = match self.b.binary_search_by(|b| b.0.cmp(&s)) {
        Ok(v) => v,
        Err(v) => v,
      };

      if ii > 0 && self.b[ii - 1].1 == s - 1 {
        self.b[ii - 1].1 = e;
        if ii < self.b.len() && self.b[ii].0 == e + 1 {
          self.b[ii - 1].1 = self.b[ii].1;
          self.b.remove(ii);
        }
      } else if ii < self.b.len() && self.b[ii].0 == e + 1 {
        self.b[ii].0 = s;
      } else {
        self.b.insert(ii, (s, e));
      }

      ans += e - s + 1;
    }
    self.m.remove(&m_id);
    ans
  }
}

fn main() {
  let mut a = Allocator::new(10);
  println!("{}", a.allocate(1, 1));
  println!("{}", a.allocate(1, 2));
  println!("{}", a.allocate(1, 3));
  println!("{}", a.free_memory(2));
  println!("{}", a.allocate(3, 4));
  println!("{}", a.allocate(1, 1));
  println!("{}", a.allocate(1, 1));
  println!("{}", a.free_memory(1));
  println!("{}", a.allocate(10, 2));
  println!("{}", a.free_memory(7));
}
