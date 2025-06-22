impl Solution {
  pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
    let k = k as usize;
    let mut bb = s.as_bytes().to_vec();
    let mut size = bb.len();

    if size % k != 0 {
      size += k - size % k;
      (0..(k - size % k)).for_each(|_| {
        bb.push(fill as u8);
      });
    }
    let mut ans: Vec<String> = vec![];
    for i in (0..size).step_by(k) {
      ans.push(String::from_utf8(bb[i..i + k].to_vec()).unwrap());
    }
    ans
  }
}
