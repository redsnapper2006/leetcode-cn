struct Solution {}

impl Solution {
  pub fn smallest_subsequence(s: String) -> String {
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

fn main() {
  println!(
    "{}",
    Solution::smallest_subsequence(String::from("cbacdcbc"))
  );
  println!(
    "{}",
    Solution::smallest_subsequence(String::from("bccab"))
  );
}
