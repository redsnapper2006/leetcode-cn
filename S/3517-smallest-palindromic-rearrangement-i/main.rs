impl Solution {
  pub fn smallest_palindrome(s: String) -> String {
    let mut buf: Vec<i32> = vec![0; 26];
    s.as_bytes().to_vec().iter().for_each(|&b| {
      buf[(b - b'a') as usize] += 1;
    });

    let mut left: Vec<u8> = vec![];
    let mut right: Vec<u8> = vec![];
    let mut odd: u8 = b'*';
    (0..26).for_each(|idx| {
      let b = b'a' + idx as u8;
      (0..buf[idx] / 2).for_each(|_| {
        left.push(b);
        right.push(b);
      });
      if buf[idx] % 2 == 1 {
        odd = b;
      }
    });

    let mut ans: Vec<u8> = left;
    if odd != b'*' {
      ans.push(odd);
    }
    right.iter().rev().for_each(|&b| {
      ans.push(b);
    });
    String::from_utf8(ans).unwrap()
  }
}
