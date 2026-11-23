struct ProductOfNumbers {
  p: Vec<i32>,
}

impl ProductOfNumbers {
  fn new() -> Self {
    ProductOfNumbers { p: vec![] }
  }

  fn add(&mut self, num: i32) {
    if num == 0 {
      self.p.clear();
    } else {
      self.p.push(*self.p.last().unwrap_or(&1) * num);
    }
  }

  fn get_product(&self, k: i32) -> i32 {
    let k = k as usize;
    let l = self.p.len();
    if l < k {
      return 0;
    }
    let div = self.p.get(l - 1 - k).copied().unwrap_or(1);
    self.p[l - 1] / div
  }
}
