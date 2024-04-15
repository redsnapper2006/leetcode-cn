const HASH: i32 = 997;

struct MyHashMap {
  b: Vec<Vec<(i32, i32)>>,
}

impl MyHashMap {
  fn new() -> Self {
    MyHashMap {
      b: vec![Vec::new(); HASH as usize],
    }
  }

  fn put(&mut self, key: i32, value: i32) {
    let offset = key % HASH;
    let offset: usize = offset as usize;

    let mut idx: usize = 0;
    while idx < self.b[offset].len() {
      if self.b[offset][idx].0 == key {
        self.b[offset][idx].1 = value;
        return;
      }
      idx += 1;
    }
    self.b[offset].push((key, value));
  }

  fn get(&self, key: i32) -> i32 {
    let offset = key % HASH;
    let offset: usize = offset as usize;

    let mut idx: usize = 0;
    while idx < self.b[offset].len() {
      if self.b[offset][idx].0 == key {
        return self.b[offset][idx].1;
      }
      idx += 1;
    }

    -1
  }

  fn remove(&mut self, key: i32) {
    let offset = key % HASH;
    let offset: usize = offset as usize;

    let mut idx: usize = 0;
    while idx < self.b[offset].len() {
      if self.b[offset][idx].0 == key {
        self.b[offset].remove(idx);
        return;
      }
      idx += 1;
    }

  }
}
