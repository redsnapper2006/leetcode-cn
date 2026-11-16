impl Solution {
  pub fn digit_sum(s: String, k: i32) -> String {
    let mut bb = s.as_bytes().iter().map(|b| (b - b'0')).collect::<Vec<u8>>();
    let k = k as usize;
    while bb.len() > k {
      let mut t: Vec<u8> = vec![];
      for chunk in bb.chunks(k) {
        let mut sum: u32 = chunk.iter().map(|&b| b as u32).sum();
        if sum == 0 {
          t.push(0);
        } else {
          let mut digits = [0u8; 6];
          let mut pos = 0usize;
          while sum > 0 {
            digits[pos] = (sum % 10) as u8;
            sum /= 10;
            pos += 1;
          }
          for i in (0..pos).rev() {
            t.push(digits[i]);
          }
        }
      }
      bb = t;
    }

    String::from_utf8(bb.iter().map(|&b| b + b'0').collect()).unwrap()
  }
}
