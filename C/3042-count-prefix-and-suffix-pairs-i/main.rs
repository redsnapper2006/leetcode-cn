impl Solution {
  pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
    let mut i: usize = 1;
    let mut ans: i32 = 0;
    while i < words.len() {
      let ib = words[i].as_bytes().to_vec();
      let mut j: usize = 0;
      while j < i {
        let jb = words[j].as_bytes().to_vec();
        if jb.len() <= ib.len() {
          let mut idx: usize = 0;
          let mut mat: bool = true;
          while idx < jb.len() {
            if ib[idx] != jb[idx] || ib[ib.len() - jb.len() + idx] != jb[idx] {
              mat = false;
              break;
            }
            idx += 1;
          }
          if mat {
            ans += 1;
          }
        }
        j += 1;
      }
      i += 1;
    }

    ans
  }
}
