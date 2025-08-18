use std::collections::BTreeSet;
use std::collections::HashMap;

struct NumberContainers {
  indexes: HashMap<i32, i32>,
  num: HashMap<i32, BTreeSet<i32>>,
}

impl NumberContainers {
  fn new() -> Self {
    NumberContainers {
      indexes: HashMap::new(),
      num: HashMap::new(),
    }
  }

  fn change(&mut self, index: i32, number: i32) {
    if self.indexes.contains_key(&index) {
      let num = self.indexes.get(&index).unwrap();
      self.num.get_mut(&num).unwrap().remove(&index);
      if self.num.get(&num).unwrap().len() == 0 {
        self.num.remove(&num);
      }
    }
    self.indexes.insert(index, number);
    self
      .num
      .entry(number)
      .and_modify(|x| {
        x.insert(index);
      })
      .or_insert(BTreeSet::from([index]));
  }

  fn find(&self, number: i32) -> i32 {
    if !self.num.contains_key(&number) {
      return -1;
    }
    *self.num.get(&number).unwrap().first().unwrap()
  }
}
