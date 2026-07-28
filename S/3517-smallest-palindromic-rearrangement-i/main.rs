impl Solution {
  pub fn smallest_palindrome(s: String) -> String {
    let bb = s.as_bytes().to_vec();
    let mut cnt: Vec<usize> = (0..bb.len() / 2).fold(vec![0; 26], |mut cnt, i| {
      cnt[(bb[i] - b'a') as usize] += 1;
      cnt
    });

    let mut ans: Vec<u8> = (0..26).fold(vec![], |mut ans, i| {
      ans.append(&mut vec![(b'a' + i) as u8; cnt[i as usize]]);
      ans
    });
    if bb.len() % 2 == 1 {
      ans.push(bb[bb.len() / 2]);
    }
    (0..bb.len() / 2).rev().for_each(|idx| {
      ans.push(ans[idx]);
    });
    String::from_utf8(ans).unwrap()
  }

  pub fn smallest_palindrome2(s: String) -> String {
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
