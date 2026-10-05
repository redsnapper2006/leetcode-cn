impl Solution {
  pub fn score_of_parentheses(s: String) -> i32 {
    let mut score: Vec<i32> = vec![];
    let mut stack: Vec<usize> = vec![];
    for b in s.bytes() {
      if b == b'(' {
        stack.push(score.len());
        score.push(0);
      } else {
        let l = stack.pop().unwrap();
        let sum: i32 = score.drain(l..).sum();
        score.push(if sum == 0 { 1 } else { 2 * sum });
      }
    }
    score.iter().sum()
  }
}
