struct Trie {
  child: [Option<Box<Trie>>; 26],
  len: usize,
  idx: usize,
}

impl Trie {
  fn new() -> Self {
    Trie {
      child: unsafe { std::mem::zeroed() },
      len: usize::MAX,
      idx: 0,
    }
  }
}

impl Solution {
  pub fn string_indices(words_container: Vec<String>, words_query: Vec<String>) -> Vec<i32> {
    const MAX_LEN: usize = 5100;

    fn build(root: &mut Trie, bb: &[u8], arr_idx: usize) {
      let mut node = root;
      let word_len = bb.len();

      if node.len > word_len {
        node.len = word_len;
        node.idx = arr_idx;
      } else if node.len == word_len && node.idx > arr_idx {
        node.idx = arr_idx;
      }

      for &b in bb.iter().rev() {
        let offset = (b - b'a') as usize;
        if node.child[offset].is_none() {
          node.child[offset] = Some(Box::new(Trie::new()));
        }
        node = node.child[offset].as_mut().unwrap();

        if node.len > word_len {
          node.len = word_len;
          node.idx = arr_idx;
        } else if node.len == word_len && node.idx > arr_idx {
          node.idx = arr_idx;
        }
      }
    }

    fn search(root: &Trie, bb: &[u8]) -> usize {
      let mut node = root;
      for &b in bb.iter().rev() {
        let offset = (b - b'a') as usize;
        match &node.child[offset] {
          Some(child) => node = child,
          None => break,
        }
      }
      node.idx
    }

    let mut root = Trie::new();
    root.len = MAX_LEN;

    for (arr_idx, wc) in words_container.into_iter().enumerate() {
      build(&mut root, wc.as_bytes(), arr_idx);
    }

    words_query.iter().map(|wq| search(&root, wq.as_bytes()) as i32).collect()
  }
}
