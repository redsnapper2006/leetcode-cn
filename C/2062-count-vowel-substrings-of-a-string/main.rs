struct Solution {}

impl Solution {
  pub fn count_vowel_substrings(word: String) -> i32 {
    let ws = word.as_bytes().to_vec();
    let mut ret: i32 = 0;
    for ii in 0..ws.len() {
      let mut a = 0;
      let mut e = 0;
      let mut i = 0;
      let mut o = 0;
      let mut u = 0;
      for j in ii..ws.len() {
        if ws[j] != 'a' as u8
          && ws[j] != 'e' as u8
          && ws[j] != 'i' as u8
          && ws[j] != 'o' as u8
          && ws[j] != 'u' as u8
        {
          break;
        }
        if ws[j] == 'a' as u8 {
          a += 1
        }
        if ws[j] == 'e' as u8 {
          e += 1
        }
        if ws[j] == 'i' as u8 {
          i += 1
        }
        if ws[j] == 'o' as u8 {
          o += 1
        }
        if ws[j] == 'u' as u8 {
          u += 1
        }
        if a > 0 && e > 0 && i > 0 && o > 0 && u > 0 {
          ret += 1
        }
      }
    }

    ret
  }
}

fn main() {
  println!("{}", Solution::count_vowel_substrings("aeiou".to_string()));
}
