struct Solution {}

impl Solution {
  pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
    let ds = dist
      .into_iter()
      .zip(speed.into_iter())
      .collect::<Vec<(i32, i32)>>();

    let mut div = ds
      .iter()
      .map(|(d, s)| (d + s - 1) / s)
      .collect::<Vec<i32>>();
    div.sort();

    let mut idx: usize = 1;
    let mut seconds: i32 = 0;
    while idx < div.len() {
      seconds += 1;
      if seconds >= div[idx] && div[idx] == div[idx - 1] {
        break;
      }
      idx += 1;
    }
    idx as i32
  }
}
