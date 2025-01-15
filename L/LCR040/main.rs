struct Solution {}

impl Solution {
  pub fn maximal_rectangle(matrix: Vec<String>) -> i32 {
    let mut zeros: Vec<Vec<i32>> = Vec::new();
    let mm = matrix.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

    mm.iter().for_each(|row| {
      let mut z: Vec<i32> = Vec::new();
      row.iter().enumerate().for_each(|(idx, &v)| {
        if v == b'0' {
          z.push(idx as i32);
        }
      });
      zeros.push(z);
    });

    let mut max: i32 = 0;
    (0..mm.len()).for_each(|row| {
      (0..mm[0].len()).for_each(|col| {
        if mm[row][col] == b'0' {
          return;
        }
        let mut maxcol: i32 = -1;
        (0..=row).rev().for_each(|lr| {
          let (offset, hit) = match zeros[lr].binary_search(&(col as i32)) {
            Ok(off) => (off, true),
            Err(off) => (off, false),
          };
          if hit {
            maxcol = maxcol.max(col as i32);
            return;
          }
          let mut lc: i32 = -1;
          if offset > 0 {
            lc = zeros[lr][offset as usize - 1];
          }
          maxcol = maxcol.max(lc);
          max = max.max((col as i32 - maxcol) * (row - lr + 1) as i32);
        });
      })
    });
    max
  }
}

fn main() {
  println!(
    "{}",
    Solution::maximal_rectangle(vec!["10100".to_string(), "10100".to_string()])
  );
  println!(
    "{}",
    Solution::maximal_rectangle(vec![
      "10100".to_string(),
      "10111".to_string(),
      "11111".to_string(),
      "10010".to_string()
    ])
  );
  println!("{}", Solution::maximal_rectangle(vec![]));
  println!("{}", Solution::maximal_rectangle(vec!["0".to_string()]));
  println!("{}", Solution::maximal_rectangle(vec!["1".to_string()]));
  println!("{}", Solution::maximal_rectangle(vec!["00".to_string()]));

  println!(
    "{}",
    Solution::maximal_rectangle(vec!["01".to_string(), "10".to_string()])
  );
  println!(
    "{}",
    Solution::maximal_rectangle(vec![
      "10111".to_string(),
      "01010".to_string(),
      "11011".to_string(),
      "11011".to_string(),
      "01111".to_string()
    ])
  );
}
