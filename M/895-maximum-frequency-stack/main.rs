use std::collections::HashMap;
struct FreqStack {
  Freq: HashMap<i32, i32>,
  Group: HashMap<i32, Vec<i32>>,
  MaxFreq: i32,
}

impl FreqStack {
  fn new() -> Self {
    FreqStack {
      Freq: HashMap::new(),
      Group: HashMap::new(),
      MaxFreq: 0,
    }
  }

  fn push(&mut self, val: i32) {
    self.Freq.entry(val).and_modify(|v| *v += 1).or_insert(1);
    let freq: &i32 = self.Freq.get(&val).unwrap();
    self
      .Group
      .entry(*freq)
      .and_modify(|v| (*v).push(val))
      .or_insert(vec![val]);
    if self.MaxFreq < *freq {
      self.MaxFreq = *freq;
    }
  }

  fn pop(&mut self) -> i32 {
    let mut max: &mut Vec<i32> = self.Group.get_mut(&self.MaxFreq).unwrap();
    let item = (*max).pop().unwrap();
    let mut freq = self.Freq.get_mut(&item).unwrap();
    *freq -= 1;
    if max.len() == 0 {
      self.Group.remove(&self.MaxFreq);
      self.MaxFreq -= 1;
    }
    item
  }
}
