impl Solution {
  pub fn total_n_queens(n: i32) -> i32 {
    fn dfs(idx: i32, n: i32, cur: &mut Vec<(i32, i32)>, ans: &mut i32) {
      if idx == n {
        *ans += 1;
        return;
      }

      (0..n).for_each(|nc| {
        let mut valid: bool = true;

        cur.iter().for_each(|&(r, c)| {
          if c == nc || (idx - r).abs() == (c - nc).abs() {
            valid = false;
          }
        });
        if valid {
          cur.push((idx, nc));
          dfs(idx + 1, n, cur, ans);
          cur.pop();
        }
      });
    }

    let mut cur: Vec<(i32, i32)> = Vec::new();
    let mut ans: i32 = 0;
    dfs(0, n, &mut cur, &mut ans);

    ans
  }
}
