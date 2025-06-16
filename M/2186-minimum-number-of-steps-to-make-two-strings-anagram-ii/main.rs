impl Solution {
  pub fn min_steps(s: String, t: String) -> i32 {
    let mut b1: Vec<i32> = vec![0; 26];
    let mut b2: Vec<i32> = vec![0; 26];

    s.as_bytes().iter().for_each(|v| {
      b1[(v - b'a') as usize] += 1;
    });
    t.as_bytes().iter().for_each(|v| {
      b2[(v - b'a') as usize] += 1;
    });

    (0..26).fold(0, |sum, idx| sum + (b1[idx] - b2[idx]).abs())
  }
}
