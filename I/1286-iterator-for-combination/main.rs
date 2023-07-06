struct CombinationIterator {
  pub B: Vec<u8>,
  pub I: Vec<usize>,
  pub K: usize,
}

impl CombinationIterator {
  fn new(characters: String, combinationLength: i32) -> Self {
    let bb: Vec<u8> = characters.bytes().collect();
    let mut idx: Vec<usize> = vec![0; combinationLength as usize];
    (0..combinationLength as usize).for_each(|i| idx[i] = i);

    let offset = idx.len() - 1;
    idx[offset] = combinationLength as usize - 2;
    CombinationIterator {
      B: bb,
      I: idx,
      K: combinationLength as usize,
    }
  }
  fn find(&mut self) -> usize {
    let mut start: usize = self.B.len();
    for offset in (0..self.K).rev() {
      if self.I[offset] + self.K - offset < self.B.len() {
        start = offset;
        break;
      }
    }

    start
  }

  fn next(&mut self) -> String {
    let start = self.find();
    self.I[start] += 1;
    (start + 1..self.K).for_each(|i| self.I[i] = self.I[i - 1] + 1);

    let mut buf: Vec<u8> = Vec::new();
    (0..self.K as usize).for_each(|i| buf.push(self.B[self.I[i]]));
    String::from_utf8(buf).unwrap()
  }

  fn has_next(&mut self) -> bool {
    self.find() != self.B.len()
  }
}

fn main() {
  let mut obj = CombinationIterator::new("abc".to_string(), 2);
  println!("{}", obj.next());
  println!("{}", obj.has_next());
  println!("{}", obj.next());
  println!("{}", obj.has_next());
  println!("{}", obj.next());
  println!("{}", obj.has_next());
}
