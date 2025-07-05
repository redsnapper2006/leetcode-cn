impl Solution {
  pub fn check_inclusion(s1: String, s2: String) -> bool {
    let mut g: Vec<i32> = vec![0; 26];
    let pp = s1.as_bytes();
    let ss = s2.as_bytes();
    pp.iter().for_each(|v| {
      g[(v - b'a') as usize] += 1;
    });

    let mut sum: Vec<i32> = vec![0; 26];
    for i in 0..ss.len() {
      sum[(ss[i] - b'a') as usize] += 1;
      if i >= pp.len() - 1 {
        if i >= pp.len() {
          sum[(ss[i - pp.len()] - b'a') as usize] -= 1;
        }
        let mut valid: bool = true;
        for ii in 0..26 {
          if sum[ii] != g[ii] {
            valid = false;
            break;
          }
        }
        if valid {
          return true;
        }
      }
    }
    false
  }
}
