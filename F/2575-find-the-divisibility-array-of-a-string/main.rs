struct Solution {}

impl Solution {
  pub fn divisibility_array(word: String, m: i32) -> Vec<i32> {
    let mut r: i64 = 0;
    word
      .as_bytes()
      .iter()
      .map(|&b| {
        let v = r * 10 + (b - b'0') as i64;
        r = v % m as i64;
        match r {
          0 => 1,
          _ => 0,
        }
      })
      .collect::<Vec<i32>>()
  }
}
