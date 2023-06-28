struct Solution {}

impl Solution {
  pub fn number_of_substrings(s: String) -> i32 {
    let mut a: i32 = -1;
    let mut b: i32 = -1;
    let mut c: i32 = -1;
    let mut res: i32 = 0;
    s.as_bytes().iter().enumerate().for_each(|(idx, &bb)| {
      match bb as char {
        'a' => {
          a = idx as i32;
        }
        'b' => {
          b = idx as i32;
        }
        _ => {
          c = idx as i32;
        }
      }

      res += a.min(b).min(c) + 1;
    });
    res
  }
}

fn main() {
  println!("{}", Solution::number_of_substrings("abcabc".to_string()));
  println!("{}", Solution::number_of_substrings("aaacb".to_string()));
}
