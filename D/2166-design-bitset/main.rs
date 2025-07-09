use std::collections::HashSet;
struct Bitset {
  size: usize,
  flip: i32,
  bit_set: HashSet<usize>,
}

impl Bitset {
  fn new(size: i32) -> Self {
    Bitset {
      size: size as usize,
      flip: 0,
      bit_set: HashSet::new(),
    }
  }

  fn fix(&mut self, idx: i32) {
    let idx = idx as usize;
    if self.flip == 0 && !self.bit_set.contains(&idx) {
      self.bit_set.insert(idx);
    }
    if self.flip == 1 && self.bit_set.contains(&idx) {
      self.bit_set.remove(&idx);
    }
  }

  fn unfix(&mut self, idx: i32) {
    let idx = idx as usize;
    if self.flip == 0 && self.bit_set.contains(&idx) {
      self.bit_set.remove(&idx);
    }
    if self.flip == 1 && !self.bit_set.contains(&idx) {
      self.bit_set.insert(idx);
    }
  }

  fn flip(&mut self) {
    self.flip = 1 - self.flip;
  }

  fn all(&self) -> bool {
    self.bit_set.len() == self.size && self.flip == 0 || self.bit_set.len() == 0 && self.flip == 1
  }

  fn one(&self) -> bool {
    self.bit_set.len() > 0 && self.flip == 0 || self.bit_set.len() != self.size && self.flip == 1
  }

  fn count(&self) -> i32 {
    if self.flip == 0 {
      self.bit_set.len() as i32
    } else {
      (self.size - self.bit_set.len()) as i32
    }
  }

  fn to_string(&self) -> String {
    let mut buf: Vec<u8> = vec![];
    for i in 0..self.size {
      buf.push(
        if self.flip == 0 && self.bit_set.contains(&i)
          || self.flip == 1 && !self.bit_set.contains(&i)
        {
          b'1'
        } else {
          b'0'
        },
      );
    }
    String::from_utf8(buf).unwrap()
  }
}

fn main() {
  let mut bs = Bitset::new(5);
  bs.fix(3);
  bs.fix(1);
  bs.flip();
  println!("{}", bs.all());
  bs.unfix(0);
  bs.flip();
  println!("{}", bs.one());
  bs.unfix(0);
  println!("{}", bs.count());
  println!("{}", bs.to_string());
}
