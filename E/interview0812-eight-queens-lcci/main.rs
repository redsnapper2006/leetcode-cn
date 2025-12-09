impl Solution {
  pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let mut ans: Vec<Vec<String>> = Vec::new();

    fn dfs(idx: i32, n: i32, cur: &mut Vec<(i32, i32)>, ans: &mut Vec<Vec<String>>) {
      if idx == n {
        let mut a: Vec<String> = Vec::new();
        cur.iter().for_each(|&(_, c)| {
          let mut t: Vec<u8> = vec![b'.'; n as usize];
          t[c as usize] = b'Q';
          a.push(String::from_utf8(t).unwrap());
        });
        ans.push(a);
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
    let mut ans: Vec<Vec<String>> = Vec::new();
    dfs(0, n, &mut cur, &mut ans);

    ans
  }
}
