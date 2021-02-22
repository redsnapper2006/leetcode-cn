struct Solution {}

impl Solution {
  pub fn print_vertically(s: String) -> Vec<String> {
    let buf: Vec<&str> = s.split(' ').collect();
    let mut bb: Vec<Vec<u8>> = Vec::new();
    let mut max = 0;
    for i in 0..buf.len() {
      if max < buf[i].len() {
        max = buf[i].len();
      }
      bb.push(buf[i].as_bytes().to_vec());
    }

    let mut m: Vec<Vec<u8>> = vec![vec![' ' as u8; buf.len()]; max];
    for i in 0..max {
      for j in 0..bb.len() {
        if i < bb[j].len() {
          m[i][j] = bb[j][i as usize];
        }
      }
    }
    let mut ret: Vec<String> = Vec::new();
    for a in &m {
      ret.push(
        String::from_utf8(a.to_vec())
          .unwrap()
          .trim_end()
          .to_string(),
      );
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::print_vertically(String::from("hell world")));
}
