impl Solution {
  pub fn distinct_subseq_ii(s: String) -> i32 {
    let bb = s.as_bytes();
    let mut dp: Vec<i64> = vec![0; s.len()];
    dp[bb.len() - 1] = 1;
    for i in (0..bb.len() - 1).rev() {
      let mut ret: i64 = 0;
      let mut hit: bool = false;
      for j in (i + 1..bb.len()) {
        if hit {
          break;
        }
        hit = bb[i] == bb[j];
        ret += dp[j];
        ret %= 1000000007;
      }

      dp[i] = (ret + if !hit { 1 } else { 0 }) % 1000000007;
    }
    (dp.iter().sum::<i64>() % 1000000007) as _
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::distinct_subseq_ii("abc".to_string()));
  println!("{}", Solution::distinct_subseq_ii("aba".to_string()));
  println!("{}", Solution::distinct_subseq_ii("aaa".to_string()));
}
