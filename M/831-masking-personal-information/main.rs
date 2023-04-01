struct Solution {}

impl Solution {
  pub fn mask_pii(s: String) -> String {
    if s.contains("@") {
      let arr: Vec<&str> = s.split('@').collect();
      // println!("{:?}",arr);
      return arr[0].get(0..1).unwrap().to_string().to_lowercase()
        + "*****"
        + &arr[0]
          .get(arr[0].len() - 1..)
          .unwrap()
          .to_string()
          .to_lowercase()
        + "@"
        + &arr[1].to_string().to_lowercase();
    }

    let buf = s
      .as_bytes()
      .iter()
      .filter(|&v| *v >= '0' as u8 && *v <= '9' as u8)
      .map(|&v| v)
      .collect::<Vec<u8>>();
    if buf.len() == 10 {
      return "***-***-".to_string() + &String::from_utf8(buf[6..].to_vec()).unwrap();
    }
    let mut prefix: Vec<u8> = vec!['*' as u8; buf.len() - 9];
    prefix[0] = '+' as u8;
    String::from_utf8(prefix).unwrap()
      + &"-***-***-".to_string()
      + &String::from_utf8(buf[buf.len() - 4..].to_vec()).unwrap()
  }
}
