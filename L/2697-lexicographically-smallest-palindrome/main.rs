struct Solution {}

impl Solution {
  pub fn make_smallest_palindrome(s: String) -> String {
    let mut bb: Vec<u8> = s.as_bytes().to_vec();
    let size: usize = bb.len();
    (0..size / 2).for_each(|i| {
      if bb[i] > bb[size - 1 - i] {
        bb[i] = bb[size - 1 - i];
      } else if bb[i] < bb[size - 1 - i] {
        bb[size - 1 - i] = bb[i];
      }
    });
    String::from_utf8(bb).unwrap()
  }
}
