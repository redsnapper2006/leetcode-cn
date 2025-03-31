impl Solution {
  pub fn percentage_letter(s: String, letter: char) -> i32 {
    s.chars()
      .filter(|&v| v == letter)
      .collect::<Vec<char>>()
      .len() as i32
      * 100
      / s.len() as i32
  }
}
