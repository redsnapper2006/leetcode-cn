struct KthLargest {
  N: Vec<i32>,
  K: i32,
}

impl KthLargest {
  fn new(k: i32, nums: Vec<i32>) -> Self {
    let mut m = Vec::new();
    m.extend_from_slice(&nums);
    m.sort();
    m.reverse();
    if m.len() > k as usize {
      m.resize(k as usize, 0);
    }
    KthLargest { K: k, N: m }
  }

  fn add(&mut self, val: i32) -> i32 {

    if self.N.len() == self.K as usize && self.N[self.N.len() - 1] >= val {
      return self.N[self.N.len() - 1];
    }
    let mut s: i32 = 0;
    let mut e: i32 = self.N.len() as i32 - 1;
    while s <= e {
      let m = s + (e - s) / 2;
      if self.N[m as usize] > val {
        s = m + 1;
      } else if self.N[m as usize] <= val {
        e = m - 1;
      }
    }
    self.N.insert(s as usize, val);
    if self.N.len() > self.K as usize {
      self.N.resize(self.K as usize, 0);
    }
    self.N[self.N.len() - 1]
  }
}

fn main() {
  let obj = KthLargest::new(3, vec![0; 5]);
  let ret_1: i32 = obj.add(3);
  println!("{}", ret_1);
}
