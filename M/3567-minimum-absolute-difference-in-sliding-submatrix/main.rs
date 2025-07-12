use std::collections::BTreeMap;
impl Solution {
  pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let mut ret: Vec<Vec<i32>> = vec![vec![0; grid[0].len() - k + 1]; grid.len() - k + 1];
    for rr in 0..grid.len() - k + 1 {
      for cc in 0..grid[0].len() - k + 1 {
        let mut buf: Vec<i32> = vec![];
        let mut btm: BTreeMap<i32, i32> = BTreeMap::new();
        for r in rr..rr + k {
          for c in cc..cc + k {
            let off = buf.partition_point(|v| v < &grid[r][c]);
            buf.insert(off, grid[r][c]);
            if off > 0 && buf[off] - buf[off - 1] != 0 {
              btm.insert(buf[off] - buf[off - 1], buf[off] - buf[off - 1]);
            }
            if off < buf.len() - 1 && buf[off + 1] - buf[off] != 0 {
              btm.insert(buf[off + 1] - buf[off], buf[off + 1] - buf[off]);
            }
          }
        }

        ret[rr][cc] = if btm.len() == 0 {
          0
        } else {
          *btm.first_key_value().unwrap().0
        };
      }
    }

    ret
  }
}
