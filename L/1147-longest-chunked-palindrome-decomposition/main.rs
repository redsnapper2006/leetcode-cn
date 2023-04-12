struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn longest_decomposition(text: String) -> i32 {
    const base1: u64 = 100000031;
    const base2: u64 = 100000037;
    const mod1: u64 = 1000000007;
    const mod2: u64 = 1000000009;

    let buf = text.as_bytes().to_vec();
    let ll = text.len();
    let mut pow1: Vec<u64> = vec![0; ll];
    let mut pow2: Vec<u64> = vec![0; ll];
    let mut pre1: Vec<u64> = vec![0; ll + 1];
    let mut pre2: Vec<u64> = vec![0; ll + 1];
    pow1[0] = 1;
    pow2[0] = 1;
    pre1[1] = (buf[0] - 'a' as u8) as u64;
    pre2[1] = pre1[1];

    (1..ll).for_each(|idx| {
      pow1[idx] = (pow1[idx - 1] * base1) % mod1;
      pow2[idx] = (pow2[idx - 1] * base2) % mod2;
      pre1[idx + 1] = (pre1[idx] * base1 + (buf[idx] - 'a' as u8) as u64) % mod1;
      pre2[idx + 1] = (pre2[idx] * base2 + (buf[idx] - 'a' as u8) as u64) % mod2;
    });

    let mut start: usize = 0;
    let mut end: usize = buf.len() - 1;

    let mut ret: i32 = 0;
    while start <= end {
      let mut len: usize = 1;
      while start + len - 1 < end - len + 1 {
        let (l1, l2) = (
          (pre1[start + len] - ((pre1[start] * pow1[start]) % mod1) + mod1) % mod1,
          (pre2[start + len] - ((pre2[start] * pow2[start]) % mod2) + mod2) % mod2,
        );
        let (r1, r2) = (
          (pre1[end + 1] - ((pre1[end - len + 1] * pow1[len]) % mod1) + mod1) % mod1,
          (pre2[end + 1] - ((pre2[end - len + 1] * pow2[len]) % mod2) + mod2) % mod2,
        );

        if l1 == r1 && l2 == r2 {
          ret += 2;
          break;
        }
        len += 1;
      }
      if start + len - 1 >= end - len + 1 {
        ret += 1;
      }
      start += len;
      end -= len;
    }
    ret
  }

  pub fn longest_decomposition2(text: String) -> i32 {
    let buf = text.as_bytes().to_vec();

    fn dfs(buf: &Vec<u8>, start: usize, end: usize) -> i32 {
      if start > end {
        return 0;
      }
      if start == end {
        return 1;
      }
      let mut diff = end - start;
      if diff % 2 == 0 {
        diff -= 1;
      }
      let mid = end - diff / 2;
      let mut idx = end;
      let mut ret: i32 = 1;

      while idx >= mid {
        if buf[start] == buf[idx] {
          let len = end - idx + 1;
          let mut is_match: bool = true;
          let mut match_idx: usize = 0;
          while match_idx < len {
            if buf[start + match_idx] != buf[idx + match_idx] {
              is_match = false;
              break;
            }
            match_idx += 1;
          }

          if is_match {
            ret = dfs(buf, start + len, idx - 1) + 2;
            break;
          }
        }

        idx -= 1;
      }
      ret
    }

    dfs(&buf, 0, buf.len() - 1)
  }
}

fn main() {
  println!(
    "{}",
    Solution::longest_decomposition("ghiabcdefhelloadamhelloabcdefghi".to_string())
  );
  println!(
    "{}",
    Solution::longest_decomposition("merchant".to_string())
  );
  println!(
    "{}",
    Solution::longest_decomposition("antaprezatepzapreanta".to_string())
  );
  println!(
    "{}",
    Solution::longest_decomposition("elvtoelvto".to_string())
  );
  println!(
    "{}",
    Solution::longest_decomposition("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string())
  );
}
