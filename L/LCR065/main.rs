use std::collections::HashSet;
impl Solution {
  pub fn minimum_length_encoding(words: Vec<String>) -> i32 {
    let mut words = words;
    words.sort_by(|x, y| {
      let lx = x.len();
      let ly = y.len();
      if lx != ly {
        return ly.cmp(&lx);
      }
      y.cmp(x)
    });

    let mut ans: i32 = 0;
    let mut m: HashSet<i64> = HashSet::new();
    for word in &words {
      let mut hash: Vec<i64> = vec![];
      let mut sum: i64 = 0;
      for b in word.as_bytes().to_vec().into_iter().rev() {
        sum = sum * 26 + (b - b'a' + 1) as i64;
        hash.push(sum);
      }
      if !m.contains(&sum) {
        for h in &hash {
          m.insert(*h);
        }
        ans += word.len() as i32 + 1;
      }
    }

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::minimum_length_encoding(vec![
      "time".to_string(),
      "me".to_string(),
      "bell".to_string()
    ])
  );
}
