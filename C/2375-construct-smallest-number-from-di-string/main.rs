impl Solution {
  pub fn smallest_number(pattern: String) -> String {
    let pp = pattern.as_bytes().to_vec();
    let mut nums: u8 = 1;
    let mut ans: Vec<u8> = vec![];
    let mut stack: u8 = 0;
    for i in 0..pp.len() {
      if pp[i] == b'I' {
        nums += stack;
        for i in 0..=stack {
          ans.push(b'0' + nums - i);
        }
        stack = 0;
        nums += 1;
      } else {
        stack += 1;
      }
    }

    nums += stack;
    for i in 0..=stack {
      ans.push(b'0' + nums - i);
    }
    String::from_utf8(ans).unwrap()
  }
}
