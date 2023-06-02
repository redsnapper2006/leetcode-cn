struct Solution {}

impl Solution {
  pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    fn is_vowel(b: u8) -> bool {
      b == 'a' as u8 || b == 'e' as u8 || b == 'i' as u8 || b == 'o' as u8 || b == 'u' as u8
    }
    let (v, _) = words.iter().fold((Vec::new(), 0), |(mut v, s), word| {
      let bb = word.as_bytes().to_vec();
      let c = match is_vowel(bb[0]) && is_vowel(bb[bb.len() - 1]) {
        true => 1,
        false => 0,
      };
      v.push(s + c);
      (v, s + c)
    });
    queries
      .iter()
      .map(|q| match (q[0] as usize, q[1] as usize) {
        (0, e) => v[e],
        (s, e) => v[e] - v[s - 1],
      })
      .collect::<Vec<i32>>()
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::vowel_strings(
      vec![
        "aba".to_string(),
        "bcb".to_string(),
        "ece".to_string(),
        "aa".to_string(),
        "e".to_string()
      ],
      vec![vec![0, 2], vec![1, 4], vec![1, 1]]
    )
  );
}
