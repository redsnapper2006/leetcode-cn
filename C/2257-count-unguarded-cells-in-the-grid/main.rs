use std::collections::HashSet;
use std::iter::Iterator;

impl Solution {
  pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 4]; n as usize]; m as usize];

    let mut gp: HashSet<(i32, i32)> = HashSet::new();
    let mut wp: HashSet<(i32, i32)> = HashSet::new();
    guards.iter().for_each(|g| {
      gp.insert((g[0], g[1]));
    });
    walls.iter().for_each(|w| {
      wp.insert((w[0], w[1]));
    });

    fn is_guard(
      dp: &mut Vec<Vec<Vec<i32>>>, idx: usize, gp: &HashSet<(i32, i32)>, wp: &HashSet<(i32, i32)>,
      first: impl Iterator<Item = i32>, second: impl Iterator<Item = i32> + Clone,
      row_or_col: bool,
    ) {
      for i in first {
        let mut is_guard: i32 = 0;
        let second = second.clone();
        for j in second {
          let pos = if row_or_col { (i, j) } else { (j, i) };
          let f = if row_or_col { i as usize } else { j as usize };
          let s = if row_or_col { j as usize } else { i as usize };
          if gp.contains(&pos) {
            is_guard = 1;
          } else if wp.contains(&pos) {
            is_guard = 0;
          } else {
            dp[f][s][idx] = is_guard;
          }
        }
      }
    }
    is_guard(&mut dp, 0, &gp, &wp, 0..m, 0..n, true);
    is_guard(&mut dp, 1, &gp, &wp, 0..m, (0..n).rev(), true);
    is_guard(&mut dp, 2, &gp, &wp, 0..n, 0..m, false);
    is_guard(&mut dp, 3, &gp, &wp, 0..n, (0..m).rev(), false);

    let mut ans: i32 = 0;
    for i in 0..m {
      for j in 0..n {
        let v = &dp[i as usize][j as usize];
        if !gp.contains(&(i, j)) && !wp.contains(&(i, j)) && v[0] + v[1] + v[2] + v[3] == 0 {
          ans += 1;
        }
      }
    }
    ans
  }
}

struct Solution {}
fn main() {
  println!(
    "{}",
    Solution::count_unguarded(
      4,
      6,
      vec![vec![0, 0], vec![1, 1], vec![2, 3]],
      vec![vec![0, 1], vec![2, 2], vec![1, 4]]
    )
  );
}
