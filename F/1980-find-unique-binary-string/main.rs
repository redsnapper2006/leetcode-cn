use std::collections::HashSet;
impl Solution {
  pub fn find_different_binary_string(nums: Vec<String>) -> String {
    String::from_utf8(nums.iter().enumerate().fold(
      vec![b'0'; nums[0].len()],
      |mut buf, (idx, num)| {
        buf[idx] = num.as_bytes()[idx] ^ 1;
        buf
      },
    ))
    .unwrap()
  }

  pub fn find_different_binary_string2(nums: Vec<String>) -> String {
    let mut m: HashSet<i32> = HashSet::new();
    nums.iter().for_each(|num| {
      m.insert(num.as_bytes().to_vec().iter().fold(0, |aggr, v| aggr * 2 + (v - b'0') as i32));
    });

    for i in 0..(1 << nums[0].len()) {
      if !m.contains(&i) {
        let mut ii = i;
        let mut buf: Vec<u8> = vec![];
        while ii > 0 {
          buf.push((ii % 2) as u8 + b'0');
          ii /= 2;
        }
        (buf.len()..nums[0].len()).for_each(|_| {
          buf.push(b'0');
        });
        buf.reverse();
        return String::from_utf8(buf).unwrap();
      }
    }
    "".to_string()
  }
}
