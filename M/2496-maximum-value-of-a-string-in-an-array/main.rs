struct Solution {}

impl Solution {
  pub fn maximum_value(strs: Vec<String>) -> i32 {
    *strs
      .iter()
      .map(|v| {
        let bb: &[u8] = v.as_bytes();
        match bb.into_iter().all(|&b| b >= '0' as u8 && b <= '9' as u8) {
          true => v.parse::<i32>().unwrap(),
          false => bb.len() as i32,
        }
      })
      .collect::<Vec<i32>>()
      .iter()
      .max()
      .unwrap()
  }
}
