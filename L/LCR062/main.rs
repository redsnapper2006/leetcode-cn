#[derive(Clone)]
struct Trie {
  child: Vec<Option<Trie>>,
  is_leaf: bool,
}

impl Trie {
  fn new() -> Self {
    Trie {
      child: vec![None; 26],
      is_leaf: false,
    }
  }

  fn insert(&mut self, word: String) {
    let bb = word.as_bytes();
    let mut p = self;
    for i in 0..bb.len() {
      let offset = (bb[i] - b'a') as usize;
      if p.child[offset].is_none() {
        p.child[offset] = Some(Trie::new());
      }
      p = p.child[offset].as_mut().unwrap();
    }
    p.is_leaf = true;
  }

  fn search(&mut self, word: String) -> bool {
    let bb = word.as_bytes();
    let mut p = self;
    for i in 0..bb.len() {
      let offset = (bb[i] - b'a') as usize;
      if p.child[offset].is_none() {
        return false;
      }
      p = p.child[offset].as_mut().unwrap();
    }
    p.is_leaf
  }

  fn starts_with(&mut self, prefix: String) -> bool {
    let bb = prefix.as_bytes();
    let mut p = self;
    for i in 0..bb.len() {
      let offset = (bb[i] - b'a') as usize;
      if p.child[offset].is_none() {
        return false;
      }
      p = p.child[offset].as_mut().unwrap();
    }
    true
  }
}
