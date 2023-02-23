struct Solution {}

impl Solution {
  pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
    let mut buf: Vec<i32> = vec![0, 1];
    let mut power: i32 = 0;

    while buf.len() < 1 << n {
      power += 1;
      let idx: usize = buf.len() - 1;
      (0..=idx).rev().for_each(|i| {
        buf.push(buf[i] + (1 << power));
      });
    }
    let mut idx: usize = 0;
    for (i, &v) in buf.iter().enumerate() {
      if v == start {
        idx = i;
        break;
      }
    }

    let mut ret: Vec<i32> = Vec::new();
    (0..buf.len()).for_each(|i| {
      ret.push(buf[(idx as usize + i) % buf.len()]);
    });
    ret
  }
}
