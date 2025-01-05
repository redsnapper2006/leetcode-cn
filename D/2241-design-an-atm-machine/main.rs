struct ATM {
  D: Vec<i32>,
  V: Vec<i32>,
}

impl ATM {
  fn new() -> Self {
    ATM {
      D: vec![0; 5],
      V: vec![20, 50, 100, 200, 500],
    }
  }

  fn deposit(&mut self, banknotes_count: Vec<i32>) {
    banknotes_count.iter().enumerate().for_each(|(idx, bc)| {
      self.D[idx] += bc;
    });
  }

  fn withdraw(&mut self, amount: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; 5];
    let mut remain = amount;
    let mut idx = self.D.len() - 1;
    while idx >= 0 {
      if self.D[idx] != 0 && self.V[idx] <= remain {
        let mut div = remain / self.V[idx];
        if div > self.D[idx] {
          div = self.D[idx];
        }
        ans[idx] = div;
        remain -= div * self.V[idx];
      }

      if remain == 0 || idx == 0 {
        break;
      }
      idx -= 1;
    }
    if remain > 0 {
      vec![-1]
    } else {
      ans.iter().enumerate().for_each(|(idx, v)| {
        self.D[idx] -= ans[idx];
      });
      ans
    }
  }
}
