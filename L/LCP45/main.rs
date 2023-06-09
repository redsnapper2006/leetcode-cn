struct Solution {}

use std::collections::HashSet;
impl Solution {
  pub fn bicycle_yard(
    position: Vec<i32>,
    terrain: Vec<Vec<i32>>,
    obstacle: Vec<Vec<i32>>,
  ) -> Vec<Vec<i32>> {
    let mut cache: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut visit: HashSet<(i32, i32)> = HashSet::new();
    let mut queue: Vec<(i32, i32, i32)> = vec![(position[0], position[1], 1)];

    let mut res: Vec<Vec<i32>> = Vec::new();

    let cord: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];
    while queue.len() > 0 {
      let (r, c, s) = queue.pop().unwrap();
      if cache.contains(&(r, c, s)) {
        continue;
      }

      cache.insert((r, c, s));
      cord.iter().for_each(|(rs, cs)| {
        if r + rs < 0
          || r + rs >= terrain.len() as i32
          || c + cs < 0
          || c + cs >= terrain[0].len() as i32
          || s + terrain[r as usize][c as usize]
            - terrain[(r + rs) as usize][(c + cs) as usize]
            - obstacle[(r + rs) as usize][(c + cs) as usize]
            <= 0
        {
          return;
        }
        let ns = s + terrain[r as usize][c as usize]
          - terrain[(r + rs) as usize][(c + cs) as usize]
          - obstacle[(r + rs) as usize][(c + cs) as usize];
        if ns == 1
          && !visit.contains(&(r + rs, c + cs))
          && (r + rs != position[0] || c + cs != position[1])
        {
          res.push(vec![r + rs, c + cs]);
          visit.insert((r + rs, c + cs));
        }
        queue.push((r + rs, c + cs, ns));
      });
    }

    res.sort_by(|x, y| {
      if x[0] == y[0] {
        return x[1].cmp(&y[1]);
      }
      x[0].cmp(&y[0])
    });
    res
  }
}

