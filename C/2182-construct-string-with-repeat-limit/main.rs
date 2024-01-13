struct Solution {}

impl Solution {
  pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
    let mut buf: Vec<i32> = vec![0; 26];
    s.as_bytes().iter().for_each(|&b| {
      let offset = (b - b'a') as usize;
      buf[offset] += 1;
    });

    let mut res: Vec<u8> = Vec::new();
    let mut idx: i32 = 25;
    while idx >= 0 {
      if buf[idx as usize] == 0 {
        idx -= 1;
        continue;
      }
      let mut count = repeat_limit;
      if count > buf[idx as usize] {
        count = buf[idx as usize];
      }
      buf[idx as usize] -= count;
      (0..count).for_each(|_| res.push(b'a' + idx as u8));

      if buf[idx as usize] > 0 {
        let mut idx2 = idx - 1;
        while idx2 >= 0 && buf[idx2 as usize] == 0 {
          idx2 -= 1;
        }
        if idx2 >= 0 {
          res.push(b'a' + idx2 as u8);
          buf[idx2 as usize] -= 1;
        } else {
          break;
        }
      } else {
        idx -= 1;
      }
    }
    String::from_utf8(res).unwrap()
  }
}

fn main() {
  println!(
    "{}",
    Solution::repeat_limited_string(String::from("cczazcc"), 3)
  );
}
