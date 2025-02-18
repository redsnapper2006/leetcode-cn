use std::collections::HashMap;

struct RangeFreqQuery {
  m: HashMap<i32, Vec<usize>>,
}

impl RangeFreqQuery {
  fn new(arr: Vec<i32>) -> Self {
    let mut m: HashMap<i32, Vec<usize>> = HashMap::new();
    arr.iter().enumerate().for_each(|(idx, &v)| {
      m.entry(v).and_modify(|x| x.push(idx)).or_insert(vec![idx]);
    });
    RangeFreqQuery { m: m }
  }

  fn query(&self, left: i32, right: i32, value: i32) -> i32 {
    if !self.m.contains_key(&value) {
      return 0;
    }

    (match self.m.get(&value).unwrap().binary_search(&(right as usize)) {
      Ok(v) => v as i32 + 1,
      Err(v) => v as i32,
    } - match self.m.get(&value).unwrap().binary_search(&(left as usize)) {
      Ok(v) => v as i32,
      Err(v) => v as i32,
    })
  }
}
