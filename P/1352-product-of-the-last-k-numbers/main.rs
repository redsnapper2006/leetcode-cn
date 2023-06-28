struct Solution {}

struct ProductOfNumbers {
  P: Vec<i32>,
}

impl ProductOfNumbers {
  fn new() -> Self {
    ProductOfNumbers { P: Vec::new() }
  }

  fn add(&mut self, num: i32) {
    if num == 0 {
      self.P = Vec::new();
      return;
    }
    let mut factor: i32 = 1;
    if self.P.len() > 0 {
      factor = self.P[self.P.len() - 1];
    }
    self.P.push(factor * num);
  }

  fn get_product(&self, k: i32) -> i32 {
    if self.P.len() < k as usize {
      return 0;
    } else if self.P.len() == k as usize {
      return self.P[self.P.len() - 1];
    }
    self.P[self.P.len() - 1] / self.P[self.P.len() - 1 - k as usize]
  }
}
