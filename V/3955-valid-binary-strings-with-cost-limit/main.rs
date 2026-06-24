impl Solution {
  pub fn generate_valid_strings(n: i32, k: i32) -> Vec<String> {
    let mut buf: Vec<(Vec<u8>, i32)> = vec![(vec![b'0'], 0), (vec![b'1'], 0)];
    (1..n).for_each(|_| {
      let size = buf.len();

      for i in 0..size {
        if buf[i].0[buf[i].0.len() - 1] != b'1' && buf[i].1 + buf[i].0.len() as i32 <= k {
          let mut t = buf[i].0.clone();
          t.push(b'1');
          buf.push((t, buf[i].1 + buf[i].0.len() as i32));
        }
        buf[i].0.push(b'0');
      }
    });
    buf.iter().map(|b| String::from_utf8((*b).0.clone()).unwrap()).collect()
  }
}
