use std::collections::HashMap;
struct FrequencyTracker {
  nf: HashMap<i32, i32>,
  fc: HashMap<i32, i32>,
}

impl FrequencyTracker {
  fn new() -> Self {
    FrequencyTracker {
      nf: HashMap::new(),
      fc: HashMap::new(),
    }
  }

  fn add(&mut self, number: i32) {
    let c = self.nf.entry(number).or_insert(0);
    if *c > 0 {
      *self.fc.get_mut(c).unwrap() -= 1;
    }

    *c += 1;
    *self.fc.entry(*c).or_insert(0) += 1;
  }

  fn delete_one(&mut self, number: i32) {
    let c = self.nf.entry(number).or_insert(0);
    if *c == 0 {
      return;
    }
    let vc = self.fc.get_mut(c).unwrap();
    *vc -= 1;

    *c -= 1;
    if *c > 0 {
      *self.fc.get_mut(c).unwrap() += 1;
    }
  }

  fn has_frequency(&self, frequency: i32) -> bool {
    if self.fc.contains_key(&frequency) && self.fc[&frequency] > 0 {
      return true;
    }
    false
  }
}
