struct Solution {}

impl Solution {
  pub fn square_is_white(coordinates: String) -> bool {
    let bb: Vec<u8> = coordinates.as_bytes().to_vec();
    let h: u8 = bb[0];
    let v = bb[1];
    !(((h - 'a' as u8) % 2 == 0) == ((v - '1' as u8) % 2 == 0))
  }
}

fn main() {
  println!("{}", Solution::square_is_white(String::from("a1")));
}
