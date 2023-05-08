impl Solution {
  pub fn rune_reserve(runes: Vec<i32>) -> i32 {
    let mut r = runes;
    r.sort();
    let mut start: usize = 0;
    let mut idx: usize = 0;
    let mut res: i32 = 0;
    while idx < r.len() - 1 {
      if r[idx + 1] - r[idx] > 1 {
        if (idx - start + 1) as i32 > res {
          res = (idx - start + 1) as i32;
        }
        start = idx + 1;
      }
      idx += 1;
    }
    if (idx - start + 1) as i32 > res {
      res = (idx - start + 1) as i32;
    }
    res
  }
}
