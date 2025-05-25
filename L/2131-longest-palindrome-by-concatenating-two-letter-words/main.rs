use std::collections::HashMap;
impl Solution {
  pub fn longest_palindrome(words: Vec<String>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();

    words.iter().for_each(|word| {
      let bb = word.as_bytes().to_vec();
      let b0 = (bb[0] - b'a') as i32;
      let b1 = (bb[1] - b'a') as i32;
      m.entry(b0 * 26 + b1).and_modify(|x| *x += 1).or_insert(1);
    });

    // println!("{:?}", m );
    let mut ans: i32 = 0;
    let mut same: bool = false;
    for (k, v) in &m {
      let b0 = k / 26;
      let b1 = k % 26;
      if b0 > b1 || !m.contains_key(&(b1 * 26 + b0)) {
        continue;
      }
      if b0 == b1 {
        ans += v / 2 * 4;
        if v % 2 == 1 {
          same = true;
        }
      } else {
        ans += v.min(m.get(&(b1 * 26 + b0)).unwrap()) * 4;
      }
    }
    // println!("{} {}", ans , same);
    if same {
      ans + 2
    } else {
      ans
    }
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::longest_palindrome(vec!["lc".to_string(), "cl".to_string(), "gg".to_string()])
  );
  println!(
    "{}",
    Solution::longest_palindrome(vec![
      "ab".to_string(),
      "ty".to_string(),
      "yt".to_string(),
      "lc".to_string(),
      "cl".to_string(),
      "ab".to_string()
    ])
  );

  println!(
    "{}",
    Solution::longest_palindrome(vec!["cc".to_string(), "ll".to_string(), "xx".to_string()])
  );
}
