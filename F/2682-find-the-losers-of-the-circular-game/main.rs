struct Solution {}

impl Solution {
  pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
    let mut buf: Vec<i32> = vec![0; n as usize];
    let mut idx: usize = 0;
    let mut offset = k;
    buf[0] = 1;
    loop {
      idx += offset as usize;
      idx %= n as usize;
      if buf[idx] == 1 {
        break;
      }
      offset += k;
      buf[idx] += 1;
    }

    buf
      .into_iter()
      .enumerate()
      .filter(|(idx, v)| *v == 0)
      .map(|(idx, _)| idx as i32 + 1)
      .collect::<Vec<i32>>()
  }
}
