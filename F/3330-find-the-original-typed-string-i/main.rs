impl Solution {
  pub fn possible_string_count(word: String) -> i32 {
    let mut ans: i32 = 0;
    let mut base: u8 = b'0';

    word.as_bytes().iter().for_each(|&b| {
      if b != base {
        base = b;
      } else {
        ans += 1;
      }
    });
    ans + 1
  }
}
