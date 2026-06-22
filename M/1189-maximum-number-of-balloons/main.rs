impl Solution {
  pub fn max_number_of_balloons(text: String) -> i32 {
    let mut buf: (i32, i32, i32, i32, i32) = (0, 0, 0, 0, 0);
    text.as_bytes().iter().for_each(|&b| match b {
      b'a' => buf.0 += 1,
      b'b' => buf.1 += 1,
      b'l' => buf.2 += 1,
      b'n' => buf.3 += 1,
      b'o' => buf.4 += 1,
      _ => {}
    });

    *[buf.0, buf.1, buf.2 / 2, buf.3, buf.4 / 2].iter().min().unwrap()
  }
}
