impl Solution {
  pub fn sum_game(num: String) -> bool {
    let bb = num.as_bytes();
    let n = bb.len();
    let mut sum: i32 = 0;
    let mut cnt: i32 = 0;
    for i in 0..n / 2 {
      match bb[i] {
        b'?' => cnt += 1,
        _ => sum += (bb[i] - b'0') as i32,
      };
      match bb[n - 1 - i] {
        b'?' => cnt -= 1,
        _ => sum -= (bb[n - 1 - i] - b'0') as i32,
      };
    }

    if cnt.abs() % 2 == 1 {
      true
    } else {
      cnt / 2 * 9 != -sum
    }
  }
}
