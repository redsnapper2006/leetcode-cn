impl Solution {
  pub fn robot_with_string(s: String) -> String {
    let mut offset: Vec<i32> = vec![-1; 26];
    let bb = s.as_bytes().to_vec();
    bb.iter().enumerate().for_each(|(idx, b)| {
      offset[(b - b'a') as usize] = idx as i32;
    });
    let mut idx: usize = 0;
    let mut stack: Vec<u8> = vec![];
    let mut ans: Vec<u8> = vec![];
    (0..26).for_each(|ii| {
      if offset[ii] == -1 {
        return;
      }

      let b = b'a' + ii as u8;

      while stack.len() > 0 && stack[stack.len() - 1] <= b {
        ans.push(stack.pop().unwrap());
      }

      while idx <= offset[ii] as usize {
        if bb[idx] == b {
          ans.push(b);
        } else {
          stack.push(bb[idx]);
        }
        idx += 1;
      }
    });
    while stack.len() > 0 {
      ans.push(stack.pop().unwrap());
    }
    String::from_utf8(ans).unwrap()
  }
}
