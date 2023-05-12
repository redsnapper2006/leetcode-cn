struct Solution {}

impl Solution {
  pub fn capture_forts(forts: Vec<i32>) -> i32 {
    let mut prev: usize = forts.len() + 1;
    let mut cur: usize = forts.len() + 1;
    let mut prev_item: i32 = 0;

    let mut res: i32 = 1;
    forts.iter().enumerate().for_each(|(idx, &v)| {
      if v == 0 {
        return;
      }

      prev = cur;
      cur = idx;
      if v != prev_item && (cur - prev) as i32 > res {
        res = (cur - prev) as i32;
      }
      prev_item = v;
    });

    res - 1
  }
}
