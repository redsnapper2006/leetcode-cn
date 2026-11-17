impl Solution {
  pub fn find_mismatched_segments(s1: String, s2: String) -> Vec<Vec<String>> {
    let bb1 = s1.as_bytes();
    let bb2 = s2.as_bytes();
    let n = bb1.len();

    let mut ans: Vec<Vec<String>> = vec![];
    let mut start: Option<usize> = None;
    for i in 0..n {
      if bb1[i] != bb2[i] {
        if start.is_none() {
          start = Some(i);
        }
      } else if let Some(s) = start.take() {
        ans.push(vec![s1[s..i].to_string(), s2[s..i].to_string()]);
      }
    }
    if let Some(s) = start.take() {
      ans.push(vec![
        s1[s..].to_string(),
        s2[s..].to_string(),
      ]);
    }
    ans
  }
}
