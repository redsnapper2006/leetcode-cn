impl Solution {
  pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
    let row = row as usize;
    let col = col as usize;
    let mut free: Vec<Vec<i32>> = vec![vec![0; col]; row];
    let mut union_set: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![vec![]; col]; row];
    for r in 0..row {
      for c in 0..col {
        union_set[r][c] = vec![(r, c), (r, c)];
      }
    }

    fn find(
      x: (usize, usize), idx: usize, union_set: &mut Vec<Vec<Vec<(usize, usize)>>>,
    ) -> (usize, usize) {
      let (r, c) = x;
      if union_set[r][c][idx] == (r, c) {
        return (r, c);
      }
      union_set[r][c][idx] = find(union_set[r][c][idx], idx, union_set);
      union_set[r][c][idx]
    }

    fn union(
      x: (usize, usize), y: (usize, usize), idx: usize,
      union_set: &mut Vec<Vec<Vec<(usize, usize)>>>,
    ) {
      let (pxr, pxc) = find(x, idx, union_set);
      let (pyr, pyc) = find(y, idx, union_set);
      if (pxr, pxc) == (pyr, pyc) {
        return;
      }
      let v = if idx == 0 {
        (pxr, pxc).min((pyr, pyc))
      } else {
        (pxr, pxc).max((pyr, pyc))
      };
      union_set[pxr][pxc][idx] = v;
      union_set[pyr][pyc][idx] = v;
    }

    let mut idx: usize = cells.len();
    for i in (0..cells.len()).rev() {
      let r = cells[i][0] as usize - 1;
      let c = cells[i][1] as usize - 1;
      free[r][c] = 1;

      for cord in vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]] {
        let nr = r as i32 + cord[0];
        let nc = c as i32 + cord[1];
        if nr < 0
          || nr >= row as i32
          || nc < 0
          || nc >= col as i32
          || free[nr as usize][nc as usize] == 0
        {
          continue;
        }
        union((r, c), (nr as usize, nc as usize), 0, &mut union_set);
        union((r, c), (nr as usize, nc as usize), 1, &mut union_set);
      }
      let mn = find((r, c), 0, &mut union_set);
      let mx = find((r, c), 1, &mut union_set);
      if mn.0 == 0 && mx.0 == row as usize - 1 {
        idx = i;
        break;
      }
    }

    idx as _
  }
}
