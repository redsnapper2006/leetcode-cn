impl Solution {
  pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
    let mut bl: Vec<i32> = vec![0; 26];
    broken_letters.as_bytes().iter().for_each(|&b| {
      bl[(b - b'a') as usize] = 1;
    });

    (text + " ")
      .as_bytes()
      .iter()
      .fold((0, true), |(sum, valid), &b| {
        if b == b' ' {
          (sum + if valid { 1 } else { 0 }, true)
        } else {
          (sum, valid && bl[(b - b'a') as usize] != 1)
        }
      })
      .0
  }
}
