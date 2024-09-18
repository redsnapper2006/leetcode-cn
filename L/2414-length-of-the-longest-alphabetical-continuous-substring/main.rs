struct Solution {}

impl Solution {
  pub fn longest_continuous_substring(s: String) -> i32 {
    let mut ans : i32 = 0;
    let bb = s.as_bytes().to_vec();
    let mut cnt : i32 = 1;
    (1..bb.len()).for_each(|idx|{
      if bb[idx] - bb[idx-1] == 1 {
        cnt +=1;
      } else {
        ans = ans.max(cnt);
        cnt = 1;
      }
    });
    ans = ans.max(cnt);
    ans
  }
}
