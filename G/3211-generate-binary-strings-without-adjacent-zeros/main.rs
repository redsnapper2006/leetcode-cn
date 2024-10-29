struct Solution {}

impl Solution {
  pub fn valid_strings(n: i32) -> Vec<String> {
    if n == 1 {
      return vec!["0".to_string(), "1".to_string()];
    }

    let mut ans: Vec<Vec<u8>> = vec![vec![b'0'], vec![b'1']];
    (1..n).for_each(|_| {
      let s = ans.len();
      (0..s).for_each(|idx| {
        if ans[idx][ans[idx].len() - 1] == b'1' {
          let mut c = ans[idx].clone();
          c.push(b'0');
          ans.push(c);
        }
        ans[idx].push(b'1');
      });
    });

    let mut ret: Vec<String> = Vec::new();
    ans.iter().for_each(|a| {
      ret.push(String::from_utf8(a.to_vec()).unwrap());
    });
    ret
  }
}
