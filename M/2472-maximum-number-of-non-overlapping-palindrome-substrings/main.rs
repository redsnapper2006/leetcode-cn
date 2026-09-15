impl Solution {
  pub fn max_palindromes(s: String, k: i32) -> i32 {
    let k = k as usize;
    let bb = s.as_bytes().to_vec();
    let mut cache: Vec<i32> = vec![-1; bb.len()];

    fn recur(bb: &[u8], k: usize, idx: usize, cache: &mut [i32]) -> i32 {
      if idx == bb.len() {
        return 0;
      }
      if cache[idx] != -1 {
        return cache[idx];
      }

      let mut ans: i32 = 0;
      for i in idx..bb.len() {
        let l = i - idx + 1;

        let mut valid: bool = true;
        if l < k {
          valid = false;
        } else {
          for j in 0..l / 2 {
            if bb[idx + j] != bb[i - j] {
              valid = false;
              break;
            }
          }
        }
        ans = ans.max(recur(bb, k, i + 1, cache) + if valid { 1 } else { 0 });
      }
      cache[idx] = ans;
      ans
    }
    for i in (0..bb.len()).rev() {
      recur(&bb, k, i, &mut cache);
    }
    cache[0]
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::max_palindromes("abaccdbbd".to_string(), 3));
  println!(
    "{}",
    Solution::max_palindromes("fttfjofpnpfydwdwdnns".to_string(), 2)
  );
}
