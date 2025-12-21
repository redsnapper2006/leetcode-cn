use std::collections::HashSet;
impl Solution {
  pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut ans: i32 = 0;
    let bbs = strs.iter().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let mut rows: HashSet<usize> = (1..bbs.len()).collect();

    let mut c: usize = 0;
    while c < bbs[0].len() && rows.len() > 0 {
      let mut valid: bool = true;
      let mut g: HashSet<usize> = HashSet::new();
      for r in &rows {
        if bbs[*r][c] > bbs[*r - 1][c] {
          g.insert(*r);
        } else if bbs[*r][c] < bbs[*r - 1][c] {
          valid = false;
          break;
        }
      }
      if !valid {
        ans += 1;
      } else {
        rows.retain(|e| !g.contains(e));
      }

      c += 1;
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::min_deletion_size(vec!["ca".to_string(), "bb".to_string(), "ac".to_string()])
  );
}
