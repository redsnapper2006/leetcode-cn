struct Solution {}
impl Solution {
  pub fn recur(buf: Vec<u8>) -> String {
    let mut low: Vec<i32> = vec![-1; 26];
    let mut upp: Vec<i32> = vec![-1; 26];
    for i in 0..buf.len() {
      if buf[i] <= 'z' as u8 && buf[i] >= 'a' as u8 {
        low[(buf[i] - 'a' as u8) as usize] = i as i32;
      }
      if buf[i] <= 'Z' as u8 && buf[i] >= 'A' as u8 {
        upp[(buf[i] - 'A' as u8) as usize] = i as i32;
      }
    }
    let mut idx: i32 = -1;
    for i in 0..26 {
      if low[i] == -1 && upp[i] == -1 {
        continue;
      }
      if low[i] > -1 && upp[i] > -1 {
        continue;
      }
      if low[i] == -1 {
        idx = upp[i] as i32;
        break;
      }
      if upp[i] == -1 {
        idx = low[i] as i32;
        break;
      }
    }
    if idx == -1 {
      return String::from_utf8(buf).unwrap();
    }
    // println!("{}", idx, );
    let (left, t) = buf.split_at(idx as usize);
    let (_, right) = t.split_at(1 as usize);
    // println!("{:?} {:?}", left, right);
    let l = Solution::recur(left.to_vec());
    let r = Solution::recur(right.to_vec());
    if l.len() >= r.len() {
      return l;
    }
    r
  }
  pub fn longest_nice_substring(s: String) -> String {
    let bb: Vec<u8> = s.as_bytes().to_vec();
    Solution::recur(bb)
  }
}
fn main() {
  println!(
    "{}",
    Solution::longest_nice_substring(String::from("YazaAay"))
  );
  println!("{}", Solution::longest_nice_substring(String::from("BbB")));
}
