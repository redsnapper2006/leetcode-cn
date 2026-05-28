impl Solution {
  pub fn resulting_string(s: String) -> String {
    let mut stack: Vec<u8> = vec![];

    s.as_bytes().iter().for_each(|&b| {
      if stack.len() == 0
        || (b as i32 - stack[stack.len() - 1] as i32 + 26).abs() % 26 != 1
          && (b as i32 - stack[stack.len() - 1] as i32 + 26).abs() % 26 != 25
      {
        stack.push(b);
      } else {
        stack.pop();
      }
    });

    String::from_utf8(stack).unwrap()
  }
}
