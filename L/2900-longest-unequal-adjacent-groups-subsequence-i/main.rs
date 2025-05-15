impl Solution {
  pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
    groups
      .iter()
      .enumerate()
      .fold((vec![], -1), |(mut ans, b), (idx, v)| {
        if b != v {
          ans.push(words[idx]);
        }
        (ans, v)
      })
      .0
  }
}
