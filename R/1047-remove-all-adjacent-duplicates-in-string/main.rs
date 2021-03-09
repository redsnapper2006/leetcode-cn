struct Solution{}

impl Solution {
  pub fn remove_duplicates(s: String) -> String {
    let mut stack :Vec<u8> = Vec::new();

    let bb = s.as_bytes();
    for i in 0..bb.len() {
      if stack.len() > 0 && bb[i] == stack[stack.len()-1] {
        stack.pop();
      } else {
          stack.push(bb[i]);
      }
    }
    String::from_utf8(stack)
  }
}

fn main() {
  println!("{}", Solution::remove_duplicates(String::from("abbaca")) );
}
