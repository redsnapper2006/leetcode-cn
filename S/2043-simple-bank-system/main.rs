struct Bank {
  b: Vec<i64>,
}

impl Bank {
  fn new(balance: Vec<i64>) -> Self {
    Bank { b: balance.clone() }
  }

  fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
    let a1 = account1 as usize;
    let a2 = account2 as usize;
    if a1 < 1
      || a1 > self.b.len()
      || a2 < 1
      || a2 > self.b.len()
      || self.b[a1 - 1] < money
    {
      return false;
    }

    self.b[a1 - 1] -= money;
    self.b[a2 - 1] += money;
    true
  }

  fn deposit(&mut self, account: i32, money: i64) -> bool {
    let a = account as usize;
    if a < 1 || a > self.b.len() {
      return false;
    }

    self.b[a - 1] += money;
    true
  }

  fn withdraw(&mut self, account: i32, money: i64) -> bool {
    let a = account as usize;
    if a < 1 || a > self.b.len() || self.b[a - 1] < money {
      return false;
    }
    self.b[a - 1] -= money;
    true
  }
}
