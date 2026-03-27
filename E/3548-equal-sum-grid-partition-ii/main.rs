use std::collections::BTreeMap;

impl Solution {
  pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
    let mut rm: BTreeMap<i64, BTreeMap<usize, Vec<usize>>> = BTreeMap::new();
    let mut cm: BTreeMap<i64, BTreeMap<usize, Vec<usize>>> = BTreeMap::new();
    let mut rows: Vec<i64> = vec![0; grid.len()];
    let mut cols: Vec<i64> = vec![0; grid[0].len()];
    let mut sum: i64 = 0;

    for r in 0..grid.len() {
      for c in 0..grid[0].len() {
        let v = grid[r][c] as i64;
        rows[r] += v;
        cols[c] += v;
        sum += v;
        rm.entry(v).or_insert(BTreeMap::new()).entry(r).or_insert(vec![]).push(c);
        cm.entry(v).or_insert(BTreeMap::new()).entry(c).or_insert(vec![]).push(r);
      }
    }

    for r in 0..grid.len() {
      rows[r] += if r > 0 { rows[r - 1] } else { 0 };
    }
    for c in 0..grid[0].len() {
      cols[c] += if c > 0 { cols[c - 1] } else { 0 };
    }

    for r in 0..grid.len() {
      if rows[r] * 2 == sum {
        return true;
      }

      if rows[r] * 2 < sum {
        let diff = sum - rows[r] * 2;

        if !rm.contains_key(&diff) {
          continue;
        }

        for (&key, value) in rm.get(&diff).unwrap().range(r + 1..) {
          if !(grid[0].len() == 1 && key > r + 1 && key < grid.len() - 1
            || r == grid.len() - 2
              && key == grid.len() - 1
              && value[0] > 0
              && value[0] < grid[0].len() - 1)
          {
            return true;
          }
        }
      } else {
        let diff = rows[r] * 2 - sum;

        if !rm.contains_key(&diff) {
          continue;
        }

        for (&key, value) in rm.get(&diff).unwrap().range(0..r + 1) {
          if !(grid[0].len() == 1 && key > 0 && key + 1 <= r
            || r == 0 && key == 0 && value[0] > 0 && value[0] < grid[0].len() - 1)
          {
            return true;
          }
        }
      }
    }
    for c in 0..grid[0].len() {
      if cols[c] * 2 == sum {
        return true;
      }

      if cols[c] * 2 < sum {
        let diff = sum - cols[c] * 2;
        if !cm.contains_key(&diff) {
          continue;
        }

        for (&key, value) in cm.get(&diff).unwrap().range(c + 1..) {
          if !(grid.len() == 1 && key > c + 1 && key < grid[0].len() - 1
            || c == grid[0].len() - 2
              && key == grid[0].len() - 1
              && value[0] > 0
              && value[0] < grid.len() - 1)
          {
            return true;
          }
        }
      } else {
        let diff = cols[c] * 2 - sum;
        if !cm.contains_key(&diff) {
          continue;
        }

        for (&key, value) in cm.get(&diff).unwrap().range(0..c + 1) {
          if !(grid.len() == 1 && key > 0 && key + 1 <= c
            || c == 0 && key == 0 && value[0] > 0 && value[0] < grid.len() - 1)
          {
            return true;
          }
        }
      }
    }
    false
  }
}
