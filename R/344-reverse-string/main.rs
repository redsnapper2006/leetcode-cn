struct Solution {}

impl Solution {
  pub fn reverse_string(s: &mut Vec<char>) {
    let mut start: usize = 0;
    let mut end: usize = s.len() - 1;
    while start < end {
      let temp = s[start];
      s[start] = s[end];
      s[end] = temp;
      start += 1;
      end -= 1;
    }
  }
}
