struct Fancy {
  b: Vec<i64>,
  o: Vec<(usize, usize, i64, i64)>,
}

impl Fancy {
  fn new() -> Self {
    Fancy {
      b: vec![],
      o: vec![],
    }
  }

  fn append(&mut self, val: i32) {
    self.b.push(val as i64);
  }

  fn add_all(&mut self, inc: i32) {
    self.o.push((self.b.len() - 1, self.o.len() + 1, 1, inc as i64));
  }

  fn mult_all(&mut self, m: i32) {
    self.o.push((self.b.len() - 1, self.o.len() + 1, m as i64, 0));
  }

  fn get_index(&mut self, idx: i32) -> i32 {
    let idx = idx as usize;
    if idx >= self.b.len() {
      return -1;
    }

    const M: i64 = 1000000007;

    fn build(idx: usize, o: &mut Vec<(usize, usize, i64, i64)>) -> (usize, i64, i64) {
      if idx == o.len() {
        return (idx, 1, 0);
      }
      const M: i64 = 1000000007;

      // (a1X + b1) * a2 + b2
      // a1*a2
      let a = o[idx].2;
      let b = o[idx].3;
      let (nxt_idx, nxt_a, nxt_b) = build(o[idx].1, o);
      o[idx].1 = nxt_idx;
      o[idx].2 = (a * nxt_a) % M;
      o[idx].3 = ((b * nxt_a) % M + nxt_b % M) % M;
      (nxt_idx, o[idx].2, o[idx].3)
    }

    let start = self.o.partition_point(|&x| x.0 < idx);
    if start == self.o.len() {
      self.b[idx] as i32
    } else {
      let (_, a, b) = build(start, &mut self.o);
      (((a * self.b[idx]) % M + b) % M) as i32
    }
  }
}

fn main() {
  let mut f: Fancy = Fancy::new();

  f.append(2);
  f.add_all(3);
  f.append(7);
  f.mult_all(2);
  println!("index 0 {}", f.get_index(0));
  f.add_all(3);
  f.append(10);
  f.mult_all(2);
  println!("index 0 {}", f.get_index(0));
  println!("index 1 {}", f.get_index(1));
  println!("index 2 {}", f.get_index(2));
}
