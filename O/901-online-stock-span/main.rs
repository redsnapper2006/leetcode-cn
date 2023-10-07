struct Solution {}

struct StockSpanner {
  b: Vec<i32>,
  i: Vec<i32>,
  c: i32,
}

impl StockSpanner {
  fn new() -> Self {
    StockSpanner {
      b: Vec::new(),
      i: Vec::new(),
      c: 0,
    }
  }

  fn next(&mut self, price: i32) -> i32 {
    self.c += 1;
    if self.b.len() == 0 || self.b[self.b.len() - 1] > price {
      self.b.push(price);
      self.i.push(self.c - 1);
      return 1;
    }

    while self.b.len() > 0 && self.b[self.b.len() - 1] <= price {
      self.b.pop();
      self.i.pop();
    }

    if self.b.len() == 0 {
      self.b.push(price);
      self.i.push(self.c - 1);
      return self.c;
    }

    self.b.push(price);
    self.i.push(self.c - 1);
    self.c - 1 - self.i[self.i.len() - 2];
  }
}
