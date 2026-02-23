impl Solution {
  pub fn has_all_codes(s: String, k: i32) -> bool {
    s.as_bytes()
      .iter()
      .enumerate()
      .scan((vec![false; 1 << k], 0usize, 0usize), |(seen, val, count), (i, &b)| {
        *val = ((*val << 1) | (b - b'0') as usize) & ((1 << k) - 1);
        if i >= k as usize - 1 && !seen[*val] {
          seen[*val] = true;
          *count += 1;
        }
        Some(*count)
      })
      .find(|&c| c == 1 << k)
      .is_some()
  }
}
