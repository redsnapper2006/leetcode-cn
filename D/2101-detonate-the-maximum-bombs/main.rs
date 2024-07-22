struct Solution {}

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
  pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
    let mut range: HashMap<usize, Vec<usize>> = HashMap::new();
    (0..bombs.len()).for_each(|idx| {
      (idx + 1..bombs.len()).for_each(|next| {
        let x = (bombs[idx][0] - bombs[next][0]).abs() as i64;
        let y = (bombs[idx][1] - bombs[next][1]).abs() as i64;
        let cr2 = bombs[idx][2] as i64;
        let nr2 = bombs[next][2] as i64;
        if x * x + y * y <= cr2 * cr2 {
          range
            .entry(idx)
            .and_modify(|nn| nn.push(next))
            .or_insert(vec![next]);
        }
        if x * x + y * y <= nr2 * nr2 {
          range
            .entry(next)
            .and_modify(|nn| nn.push(idx))
            .or_insert(vec![idx]);
        }
      });
    });
    // println!("{:?}", range);
    // let mut dp: Vec<i32> = vec![0; bombs.len()];
    let mut ans: i32 = 0;
    fn dfs(
      idx: usize,
      range: &HashMap<usize, Vec<usize>>,
      visited: &mut HashSet<usize>,
      ans: &mut i32,
    ) {
      visited.insert(idx);

      let v = range.get(&idx);
      if v.is_some() {
        range.get(&idx).unwrap().into_iter().for_each(|next| {
          if visited.contains(&next) {
            return;
          }
          dfs(*next, range, visited, ans);
        });
      }

      if *ans < visited.len() as i32 {
        *ans = visited.len() as i32;
      }
    }

    (0..bombs.len()).for_each(|idx| {
      let mut visited: HashSet<usize> = HashSet::new();
      dfs(idx, &mut range, &mut visited, &mut ans);
    });

    ans
  }
}

fn main() {
  println!(
    "{}",
    Solution::maximum_detonation(vec![vec![2, 1, 3], vec![6, 1, 4]])
  );
  println!(
    "{}",
    Solution::maximum_detonation(vec![vec![1, 1, 5], vec![10, 10, 5]])
  );
  println!(
    "{}",
    Solution::maximum_detonation(vec![
      vec![1, 2, 3],
      vec![2, 3, 1],
      vec![3, 4, 2],
      vec![4, 5, 3],
      vec![5, 6, 4]
    ])
  );
  println!(
    "{}",
    Solution::maximum_detonation(vec![vec![1, 1, 100000], vec![100000, 100000, 1]])
  );
}
