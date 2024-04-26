struct SnapshotArray {
  snap_id: i32,
  arr: Vec<Vec<(i32, i32)>>,
}

impl SnapshotArray {
  fn new(length: i32) -> Self {
    SnapshotArray {
      snap_id: 0,
      arr: vec![Vec::new(); length as usize],
    }
  }

  fn set(&mut self, index: i32, val: i32) {
    let index = index as usize;
    let size = self.arr[index].len();
    if size == 0 || self.arr[index][size - 1].0 < self.snap_id {
      self.arr[index].push((self.snap_id, val));
    } else {
      self.arr[index][size - 1].1 = val;
    }
  }

  fn snap(&mut self) -> i32 {
    self.snap_id += 1;
    self.snap_id - 1
  }

  fn get(&self, index: i32, snap_id: i32) -> i32 {
    let index = index as usize;
    match self.arr[index].binary_search_by(|x| x.0.cmp(&snap_id)) {
      Ok(i) => self.arr[index][i].1,
      Err(i) => self.arr[index].get(i - 1).map_or(0, |x| x.1),
    }
  }
}
