use std::collections::HashMap;
struct Spreadsheet {
  m: HashMap<String, i32>,
}

impl Spreadsheet {
  fn new(rows: i32) -> Self {
    Spreadsheet { m: HashMap::new() }
  }

  fn set_cell(&mut self, cell: String, value: i32) {
    self.m.insert(cell, value);
  }

  fn reset_cell(&mut self, cell: String) {
    self.m.insert(cell, 0);
  }

  fn get_value(&self, formula: String) -> i32 {
    let bb = formula.as_bytes().to_vec();
    let mut b1: Vec<u8> = vec![];
    let mut b2: Vec<u8> = vec![];
    let mut stage: bool = true;
    for i in 1..bb.len() {
      if bb[i] == b'+' {
        stage = false;
        continue;
      }
      if stage {
        b1.push(bb[i]);
      } else {
        b2.push(bb[i]);
      }
    }

    fn cv(b1: &Vec<u8>, m: &HashMap<String, i32>) -> i32 {
      let mut b1d: i32 = 0;
      if b1[0] >= b'A' && b1[0] <= b'Z' {
        let c = String::from_utf8(b1.clone()).unwrap();
        if m.contains_key(&c) {
          b1d = *m.get(&c).unwrap();
        }
      } else {
        for i in 0..b1.len() {
          b1d *= 10;
          b1d += (b1[i] - b'0') as i32;
        }
      }
      b1d
    }
    cv(&b1, &self.m) + cv(&b2, &self.m)
  }
}
