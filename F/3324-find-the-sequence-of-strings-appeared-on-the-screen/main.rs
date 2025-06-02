impl Solution {
  pub fn string_sequence(target: String) -> Vec<String> {
    let mut ans : Vec<String> = vec![];
    let mut t : Vec<u8> = vec![];
    target.as_bytes().iter().for_each(|&b| {
      let mut s : u8 = b'a';
      while s <= b {
        let mut tt = t.clone();
        tt.push(s);
        ans.push(String::from_utf8(tt).unwrap());
        s += 1;
      }
      t.push(b);
    });
    ans
  }
}
