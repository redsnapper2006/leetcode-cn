impl Solution {
  pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut ans: i32 = 0;
    let bbs = strs.iter().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    for c in 0..bbs[0].len() {
      let mut valid: bool = true;
      for r in 1..bbs.len() {
        if bbs[r][c] < bbs[r - 1][c] {
          valid = false;
          break;
        }
      }
      if !valid {
        ans += 1;
      }
    }
    ans
  }
}
