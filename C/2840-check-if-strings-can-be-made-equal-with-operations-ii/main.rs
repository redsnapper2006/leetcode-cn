impl Solution {
  pub fn check_strings(s1: String, s2: String) -> bool {
    let mut odd: Vec<i32> = vec![0; 26];
    let mut even: Vec<i32> = vec![0; 26];
    s1.as_bytes().iter().enumerate().for_each(|(idx, &v)| {
      let offset = (v - b'a') as usize;
      match idx % 2 {
        1 => odd[offset] += 1,
        _ => even[offset] += 1,
      };
    });

    s2.as_bytes().iter().enumerate().for_each(|(idx, &v)| {
      let offset = (v - b'a') as usize;
      match idx % 2 {
        1 => odd[offset] -= 1,
        _ => even[offset] -= 1,
      };
    });

    let mut idx: usize = 0;
    while idx < 26 {
      if odd[idx] != 0 || even[idx] != 0 {
        return false;
      }
      idx += 1;
    }
    true
  }
}
