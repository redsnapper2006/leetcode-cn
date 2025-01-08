struct Solution {}

impl Solution {
  pub fn largest_good_integer(num: String) -> String {
    let bb = num.as_bytes().to_vec();
    let mut idx: usize = 2;

    let mut c: u8 = b' ';
    while idx < bb.len() {
      if bb[idx - 2] == bb[idx - 1] && bb[idx - 1] == bb[idx] {
        if (c == b' ' || c < bb[idx]) {
          c = bb[idx];
        }
        idx += 3;
      } else {
        idx += 1;
      }
    }

    if c == b' ' {
      "".to_string()
    } else {
      String::from_utf8(vec![c, c, c]).unwrap()
    }
  }
}
