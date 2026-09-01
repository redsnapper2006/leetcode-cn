use std::collections::HashMap;

struct SQL {
  data: HashMap<String, Vec<(i32, Vec<String>)>>,
  meta: HashMap<String, (u8, i32)>,
}

impl SQL {
  fn new(names: Vec<String>, columns: Vec<i32>) -> Self {
    let mut d: HashMap<String, Vec<(i32, Vec<String>)>> = HashMap::new();
    let mut m: HashMap<String, (u8, i32)> = HashMap::new();
    for (idx, n) in names.iter().enumerate() {
      d.insert(n.clone(), vec![]);
      m.insert(n.clone(), (columns[idx] as u8, 0));
    }
    SQL { data: d, meta: m }
  }

  fn name_valid(&self, name: &String) -> bool {
    self.meta.contains_key(name)
  }

  fn column_valid(&self, name: &String, len: usize) -> bool {
    self.meta.get(name).unwrap().0 == len as u8
  }

  fn columnid_valid(&self, name: &String, id: i32) -> bool {
    println!("cid_valid {} {}", self.meta.get(name).unwrap().0, name);
    self.meta.get(name).unwrap().0 >= (id as u8)
  }

  fn id_valid(&self, name: &String, id: i32) -> (usize, bool) {
    match self.data.get(name).unwrap().binary_search_by(|p| p.0.cmp(&id)) {
      Ok(ov) => (ov, true),
      Err(ev) => (ev, false),
    }
  }

  fn ins(&mut self, name: String, row: Vec<String>) -> bool {
    if !self.name_valid(&name) || !self.column_valid(&name, row.len()) {
      return false;
    }

    let meta = self.meta.get_mut(&name).unwrap();
    meta.1 += 1;
    self.data.get_mut(&name).unwrap().push((meta.1, row.clone()));
    true
  }

  fn rmv(&mut self, name: String, row_id: i32) {
    if !self.name_valid(&name) {
      return;
    }
    let (idx, exist) = self.id_valid(&name, row_id);
    if !exist {
      return;
    }
    self.data.get_mut(&name).unwrap().remove(idx);
  }

  fn sel(&self, name: String, row_id: i32, column_id: i32) -> String {
    if !self.name_valid(&name) || !self.columnid_valid(&name, column_id) {
      return "<null>".to_string();
    }
    let (idx, exist) = self.id_valid(&name, row_id);
    if !exist {
      return "<null>".to_string();
    }
    self.data.get(&name).unwrap()[idx].1[column_id as usize - 1].clone()
  }

  fn exp(&self, name: String) -> Vec<String> {
    if !self.name_valid(&name) {
      return vec![];
    }
    self
      .data
      .get(&name)
      .unwrap()
      .iter()
      .map(|r| format!("{},{}", r.0, r.1.join(",")))
      .collect::<Vec<String>>()
  }
}
