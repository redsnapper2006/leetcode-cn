use std::collections::HashMap;
impl Solution {
  pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut prev: Vec<HashMap<i32, i32>> = vec![HashMap::new(); grid[0].len()];

    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        let (score, cost) = match grid[r][c] {
          0 => (0, 0),
          1 => (1, 1),
          _ => (2, 1),
        };
        if r == 0 && c == 0 {
          prev[0] = HashMap::from([(cost, score)]);
          continue;
        }

        let mut cur: HashMap<i32, i32> = HashMap::new();
        let left: HashMap<i32, i32> = if c > 0 { prev[c - 1].clone() } else { HashMap::new() };
        for (lc, ls) in left.iter() {
          if lc + cost > k {
            continue;
          }
          cur.insert(lc + cost, ls + score);
        }
        let up: HashMap<i32, i32> = if r > 0 { prev[c].clone() } else { HashMap::new() };
        for (uc, us) in up.iter() {
          if uc + cost > k || cur.contains_key(&(uc + cost)) && *cur.get(&(uc + cost)).unwrap() > us + score {
            continue;
          }
          cur.insert(uc + cost, us + score);
        }

        prev[c] = cur.clone();
      }
    }

    let mut ans: i32 = -1;
    for (_, s) in prev[grid[0].len() - 1].iter() {
      ans = ans.max(*s);
    }
    ans
  }
}
