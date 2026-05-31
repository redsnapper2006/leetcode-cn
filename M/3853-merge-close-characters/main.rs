impl Solution {
  pub fn merge_characters(s: String, k: i32) -> String {
    let mut stack: Vec<u8> = vec![];
    let mut pos: Vec<i32> = vec![-1; 26];
    let bb = s.as_bytes().to_vec();
    bb.iter().for_each(|&b| {
      let off = (b - b'a') as usize;
      if stack.len() == 0 || pos[off] == -1 || stack.len() as i32 - pos[off] > k {
        stack.push(b);
        pos[off] = stack.len() as i32 - 1;
      }
    });
    String::from_utf8(stack).unwrap()
  }
}
