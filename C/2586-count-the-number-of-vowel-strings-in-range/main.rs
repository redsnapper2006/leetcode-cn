struct Solution {}

impl Solution {
  pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
    (left..=right)
      .map(|idx| {
        let bb = words[idx as usize].as_bytes().to_vec();
        let f = match bb[0] {
          b'a' => true,
          b'e' => true,
          b'i' => true,
          b'o' => true,
          b'u' => true,
          _ => false,
        };

        let l = match bb[bb.len() - 1] {
          b'a' => true,
          b'e' => true,
          b'i' => true,
          b'o' => true,
          b'u' => true,
          _ => false,
        };
        if f && l {
          1
        } else {
          0
        }
      })
      .collect::<Vec<i32>>()
      .iter()
      .sum()
  }
}
