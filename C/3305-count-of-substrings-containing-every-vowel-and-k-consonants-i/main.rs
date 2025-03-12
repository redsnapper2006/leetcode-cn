struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn count_of_substrings(word: String, k: i32) -> i32 {
    fn calc(
      b: u8,
      a: &mut i32,
      e: &mut i32,
      i: &mut i32,
      o: &mut i32,
      u: &mut i32,
      other: &mut i32,
      f: i32,
    ) {
      if b == b'a' {
        *a += f;
      } else if b == b'e' {
        *e += f;
      } else if b == b'i' {
        *i += f;
      } else if b == b'o' {
        *o += f;
      } else if b == b'u' {
        *u += f;
      } else {
        *other += f;
      }
    }

    fn valid(a: i32, e: i32, i: i32, o: i32, u: i32, other: i32, prev: &Vec<i32>, k: i32) -> bool {
      a - prev[0] > 0
        && e - prev[1] > 0
        && i - prev[2] > 0
        && o - prev[3] > 0
        && u - prev[4] > 0
        && other - prev[5] == k
    }

    let bb = word.as_bytes().to_vec();
    let mut m: HashMap<i32, i32> = HashMap::new();
    let mut dp: Vec<Vec<i32>> = vec![];
    let mut ans: i32 = 0;
    let mut a: i32 = 0;
    let mut e: i32 = 0;
    let mut i: i32 = 0;
    let mut o: i32 = 0;
    let mut u: i32 = 0;
    let mut other: i32 = 0;

    m.insert(0, 0);
    dp.push(vec![0, 0, 0, 0, 0, 0]);
    (0..bb.len()).for_each(|idx| {
      calc(
        bb[idx], &mut a, &mut e, &mut i, &mut o, &mut u, &mut other, 1,
      );
      if !m.contains_key(&other) {
        m.insert(other, idx as i32 + 1);
      }
      dp.push(vec![a, e, i, o, u, other]);

      if m.contains_key(&(other - k)) {
        let mut start = *m.get(&(other - k)).unwrap();
        let mut end = idx as i32 + 1;
        while start <= end {
          let m = start + (end - start) / 2;
          if valid(a, e, i, o, u, other, &dp[m as usize], k) {
            start = m + 1;
          } else {
            end = m - 1
          }
        }
        ans += start - *m.get(&(other - k)).unwrap();
      }
    });
    ans
  }
}

fn main() {
  println!("{}", Solution::count_of_substrings("aeioqq".to_string(), 1));
  println!("{}", Solution::count_of_substrings("aeiou".to_string(), 0));
  println!(
    "{}",
    Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1)
  );
  println!(
    "{}",
    Solution::count_of_substrings("iqeaouqi".to_string(), 2)
  );
  println!(
    "{}",
    Solution::count_of_substrings("ieiaoud".to_string(), 0)
  );
  println!(
    "{}",
    Solution::count_of_substrings("aaeuoiouee".to_string(), 0)
  );
}
