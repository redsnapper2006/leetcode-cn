impl Solution {
  pub fn min_rotations(n: i32, s: String) -> i32 {
    let n = n as usize;
    let bb: Vec<i32> = s.as_bytes().iter().map(|b| (b - b'0') as i32).collect::<Vec<i32>>();
    let mut left: Vec<i32> = vec![0; n];
    let mut right: Vec<i32> = vec![0; n];
    let mut prev = 0i32;
    for i in 0..n {
      let diff = (bb[i] - prev).abs();
      left[i] = diff.min(10 - diff) + if i > 0 { left[i - 1] } else { 0 };
      prev = bb[i];
    }
    prev = bb[n - 1];
    for i in (0..n - 1).rev() {
      let diff = (bb[i] - prev).abs();
      right[i] = right[i + 1] + diff.min(10 - diff);
      prev = bb[i];
    }

    let mut ans = bb[n - 1].min(10 - bb[n - 1]) + right[0];
    for i in 0..n - 1 {
      let diff = (bb[i] - bb[n - 1]).abs();
      ans = ans.min(left[i] + diff.min(10 - diff) + right[i + 1])
    }
    ans
  }
}
