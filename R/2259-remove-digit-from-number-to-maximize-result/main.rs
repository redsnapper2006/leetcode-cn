struct Solution {}

impl Solution {
  pub fn remove_digit(number: String, digit: char) -> String {
    let mut nn: Vec<u8> = number.as_bytes().to_vec();
    let mut idx: Vec<usize> = Vec::new();
    for (i, d) in nn.iter().enumerate() {
      if *d == digit as u8 {
        idx.push(i);
      }
    }
    let mut t: usize = idx[idx.len() - 1];
    for i in idx {
      if i < number.len() - 1 && nn[i + 1] > nn[i] {
        t = i;
        break;
      }
    }
    nn.remove(t);
    String::from_utf8(nn).unwrap()
  }
}
