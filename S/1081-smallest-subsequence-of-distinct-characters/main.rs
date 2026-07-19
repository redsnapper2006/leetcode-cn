impl Solution {
  pub fn smallest_subsequence(s: String) -> String {
    let mut cnt: Vec<i32> = vec![0; 26];

    let bb = s.as_bytes().to_vec();
    bb.iter().for_each(|&b| {
      cnt[(b - b'a') as usize] += 1;
    });

    let mut stack: Vec<u8> = vec![];
    let mut visited: i32 = 0;
    bb.iter().for_each(|&b| {
      let offset = (b - b'a') as usize;
      cnt[offset] -= 1;
      if visited & (1 << offset) > 0 {
        return;
      }

      while !stack.is_empty()
        && *stack.last().unwrap() > b
        && cnt[(*stack.last().unwrap() - b'a') as usize] > 0
      {
        let prev = stack.pop().unwrap();
        visited ^= 1 << ((prev - b'a') as i32);
      }
      stack.push(b);
      visited |= 1 << offset;
    });

    String::from_utf8(stack).unwrap()
  }

  pub fn smallest_subsequence2(s: String) -> String {
    let mut visit: Vec<i32> = vec![0; 26];
    let mut stack: Vec<u8> = Vec::new();

    let bb = s.as_bytes().to_vec();
    let mut cnt: Vec<i32> = vec![0; 26];
    for i in 0..bb.len() {
      cnt[(bb[i] - 'a' as u8) as usize] += 1;
    }

    for i in 0..bb.len() {
      let idx = (bb[i] - 'a' as u8) as usize;
      if visit[idx] == 1 {
        cnt[idx] -= 1;
        continue;
      }
      while stack.len() > 0 && stack[stack.len() - 1] > bb[i] {
        let idx = stack[stack.len() - 1] - 'a' as u8;
        if cnt[idx as usize] > 0 {
          visit[idx as usize] = 0;
          stack.remove(stack.len() - 1);
        } else {
          break;
        }
      }
      cnt[idx] -= 1;
      visit[idx] = 1;
      stack.push(bb[i]);
    }
    String::from_utf8(stack).unwrap()
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::smallest_subsequence(String::from("cbacdcbc"))
  );
  println!("{}", Solution::smallest_subsequence(String::from("bccab")));
}
