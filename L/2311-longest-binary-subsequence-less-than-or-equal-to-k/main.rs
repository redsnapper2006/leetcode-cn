impl Solution {
  pub fn longest_subsequence(s: String, k: i32) -> i32 {
    let bb = s.as_bytes().to_vec();

    let mut start: usize = 0;
    let mut n: i32 = 0;
    let mut removed : i32 = 0;
    let mut ans: i32 = 0;
    for i in 0..bb.len() {
      n = n * 2 + (bb[i] - b'0') as i32;
      if n > k {
        while start <= i && n > k {
          if bb[start] == b'1' {
            n -= 1 << (i - start);
            removed += 1;
          }
          start += 1;
        }
      }

      ans = ans.max(i  as i32 - removed +1  );
    }

    ans
  }
}
