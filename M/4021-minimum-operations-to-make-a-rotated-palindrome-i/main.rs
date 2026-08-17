impl Solution {
  pub fn min_operations(s: String) -> i32 {
    let n = s.len();
    let bb: Vec<u8> = s.bytes().chain(s.bytes()).collect();

    let mut ans: i32 = i32::MAX;

    let sum = |start: usize, end: usize| -> i32 {
      let mut s = start;
      let mut e = end;
      let mut ans: i32 = 0;
      while s < e {
        let diff = (bb[s] as i32 - bb[e] as i32).abs();
        ans += diff.min(26 - diff);
        s += 1;
        e -= 1;
      }
      ans
    };

    for i in 0..n {
      ans = ans.min(sum(i, i + n - 1) + i as i32);
    }
    ans
  }
}
