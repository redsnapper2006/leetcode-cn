impl Solution {
  pub fn maximum_xor(s: String, t: String) -> String {
    let mut ans: Vec<u8> = vec![];
    let mut cnt: Vec<i32> = vec![0, 0];
    t.as_bytes().iter().for_each(|&b| {
      cnt[(b - b'0') as usize] += 1;
    });

    let bb = s.as_bytes().to_vec();
    for i in 0..bb.len() {
      let off = (b'1' - bb[i]) as usize;
      ans.push(if cnt[off] > 0 {
        cnt[off] -= 1;
        b'1'
      } else {
        cnt[1 - off] -= 1;
        b'0'
      });
    }
    String::from_utf8(ans).unwrap()
  }
}
