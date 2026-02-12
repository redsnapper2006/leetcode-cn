impl Solution {
  pub fn longest_balanced(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();

    let mut sum: Vec<i32> = vec![0; 26];
    let mut aggr: Vec<Vec<i32>> = vec![vec![]; bb.len()];
    let mut ans: usize = 0;
    for i in 0..bb.len() {
      let offset = (bb[i] - b'a') as usize;
      sum[offset] += 1;
      aggr[i] = sum.clone();

      for j in 0..=(i - ans) {
        let mut cnt: i32 = 0;
        let mut valid: bool = true;
        for m in 0..26 {
          let diff: i32 = aggr[i][m] - if j == 0 { 0 } else { aggr[j - 1][m] };
          if diff == 0 {
            continue;
          } else if cnt == 0 {
            cnt = diff;
          } else if cnt != diff {
            valid = false;
            break;
          }
        }

        if valid {
          ans = ans.max(i - j + 1);
          break;
        }
      }
    }

    ans as _
  }
}

struct Solution {}
fn main() {
  println!("{}", Solution::longest_balanced("zzabccy".to_string()));
  println!("{}", Solution::longest_balanced("kooo".to_string()));
  println!("{}", Solution::longest_balanced("aba".to_string()));
}
