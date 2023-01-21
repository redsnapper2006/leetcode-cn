struct Solution {}

impl Solution {
  pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; 3]; obstacles.len()];
    buf[0][0] = 1;
    buf[0][2] = 1;

    (1..obstacles.len()).for_each(|oidx| {
      if obstacles[oidx] == 0 {
        (0..3).for_each(|sidx| {
          buf[oidx][sidx] = buf[oidx - 1][sidx];
        });
        let min = buf[oidx].clone().into_iter().min().unwrap();
        (0..3).for_each(|sidx| {
          if buf[oidx][sidx] > min + 1 {
            buf[oidx][sidx] = min + 1;
          }
        });
      } else {
        let offset = obstacles[oidx] - 1;
        buf[oidx][offset as usize] = 1 << 31 - 1;
        let mut i1: usize;
        let mut i2: usize;
        if offset == 0 {
          i1 = 1;
          i2 = 2;
        } else if offset == 1 {
          i1 = 0;
          i2 = 2;
        } else {
          i1 = 0;
          i2 = 1;
        }
        if buf[oidx - 1][i1] != 1 << 31 - 1 {
          buf[oidx][i1] = buf[oidx - 1][i1];
        }
        if buf[oidx - 1][i2] != 1 << 31 - 1 {
          buf[oidx][i2] = buf[oidx - 1][i2];
        }
        if buf[oidx - 1][i1] == 1 << 31 - 1 {
          buf[oidx][i1] = buf[oidx][i2] + 1;
        }
        if buf[oidx - 1][i2] == 1 << 31 - 1 {
          buf[oidx][i2] = buf[oidx][i1] + 1;
        }
      }
    });
    *buf[obstacles.len() - 1].iter().min().unwrap()
  }
}
