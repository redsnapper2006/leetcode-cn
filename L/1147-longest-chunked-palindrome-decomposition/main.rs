struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn longest_decomposition(text: String) -> i32 {
    let buf = text.as_bytes().to_vec();
    let mut cached: HashMap<usize, HashMap<usize, i32>> = HashMap::new();

    fn dfs(
      buf: &Vec<u8>,
      start: usize,
      end: usize,
      cached: &mut HashMap<usize, HashMap<usize, i32>>,
    ) -> i32 {
      if cached.contains_key(&start) && cached.get(&start).unwrap().contains_key(&end) {
        return *cached.get(&start).unwrap().get(&end).unwrap();
      }
      if start > end {
        if !cached.contains_key(&start) {
          cached.insert(start, HashMap::new());
        }
        cached.get_mut(&start).unwrap().insert(end, 0);
        return 0;
      }
      if start == end {
        if !cached.contains_key(&start) {
          cached.insert(start, HashMap::new());
        }
        cached.get_mut(&start).unwrap().insert(end, 1);
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
            let r1 = dfs(buf, start + len, idx - 1, cached);
            if ret < r1 + 2 {
              ret = r1 + 2;
            }
          }
        }

        idx -= 1;
      }
      if !cached.contains_key(&start) {
        cached.insert(start, HashMap::new());
      }
      cached.get_mut(&start).unwrap().insert(end, ret);
      ret
    }

    dfs(&buf, 0, buf.len() - 1, &mut cached)
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
