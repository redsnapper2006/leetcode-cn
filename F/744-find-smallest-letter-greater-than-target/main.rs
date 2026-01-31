impl Solution {
  pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut s: usize = 0;
    let mut e: usize = letters.len();
    while s < e {
      let m = s + (e - s) / 2;
      if letters[m] > target {
        e = e - 1;
      } else {
        s = s + 1;
      }
    }
    if s == letters.len() {
      letters[0]
    } else {
      letters[s]
    }
  }
}
