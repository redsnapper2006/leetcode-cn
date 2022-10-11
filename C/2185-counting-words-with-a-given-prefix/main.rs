struct Solution {}

impl Solution {
  pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    let n: usize = pref.len();
    let mut ret: i32 = 0;
    for w in words {
      if w.len() >= n && w.as_str().starts_with(pref.as_str()) {
        ret += 1
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::prefix_count(
      vec![
        "pay".to_string(),
        "attention".to_string(),
        "practice".to_string(),
        "attend".to_string()
      ],
      "at".to_string()
    )
  )
}
