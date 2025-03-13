struct Solution {}

impl Solution {
  pub fn count_of_substrings(word: String, k: i32) -> i64 {
    fn sum(
      b: u8,
      a: &mut i64,
      e: &mut i64,
      i: &mut i64,
      o: &mut i64,
      u: &mut i64,
      other: &mut i64,
      f: i64,
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

    let bb = word.as_bytes().to_vec();
    let k = k as i64;
    fn calc(bb: &Vec<u8>, k: i64) -> i64 {
      let mut ans: i64 = 0;
      let mut a: i64 = 0;
      let mut e: i64 = 0;
      let mut i: i64 = 0;
      let mut o: i64 = 0;
      let mut u: i64 = 0;
      let mut other: i64 = 0;
      let mut start: usize = 0;
      let mut idx: usize = 0;
      while start < bb.len() {
        while idx < bb.len() && (other < k || a < 1 || e < 1 || i < 1 || o < 1 || u < 1) {
          sum(
            bb[idx], &mut a, &mut e, &mut i, &mut o, &mut u, &mut other, 1,
          );
          idx += 1;
        }
        if other >= k && a > 0 && e > 0 && i > 0 && o > 0 && u > 0 {
          ans += (bb.len() - idx + 1) as i64;
        }
        sum(
          bb[start], &mut a, &mut e, &mut i, &mut o, &mut u, &mut other, -1,
        );
        start += 1;
      }
      ans
    }

    (calc(&bb, k) - calc(&bb, k + 1)) as _
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
