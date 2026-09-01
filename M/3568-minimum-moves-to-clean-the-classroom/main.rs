use std::collections::VecDeque;

impl Solution {
  pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
    let cr = classroom.iter().map(|r| r.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let (r, c) = (cr.len(), cr[0].len());

    let mut idx: Vec<Vec<u16>> = vec![vec![0; c]; r];
    let mut cnt: u16 = 0;
    let (mut x, mut y) = (0i8, 0i8);
    for i in 0..r {
      for j in 0..c {
        if cr[i][j] == b'L' {
          idx[i][j] = (1 << cnt);
          cnt += 1;
        }
        if cr[i][j] == b'S' {
          x = i as i8;
          y = j as i8;
        }
      }
    }

    let full = (1u16 << cnt) - 1;
    const CORDS: [[i8; 2]; 4] = [[-1i8, 0i8], [1i8, 0i8], [0i8, -1i8], [0i8, 1i8]];
    let (ri, ci) = (r as i8, c as i8);

    let mut visit: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; 1 << cnt]; c]; r];
    let mut q: VecDeque<(i8, i8, i32, u16, i32)> = VecDeque::new();
    q.push_back((x, y, energy, 0, 0));

    while let Some((x, y, e, m, s)) = q.pop_front() {
      if m == full {
        return s;
      }
      if e == 0 {
        continue;
      }
      for cord in CORDS {
        let (nx, ny) = (x + cord[0], y + cord[1]);
        if nx < 0 || nx >= ri || ny < 0 || ny >= ci {
          continue;
        }
        let (ux, uy) = (nx as usize, ny as usize);
        if cr[ux][uy] == b'X' {
          continue;
        }
        let ne = if cr[ux][uy] == b'R' { energy } else { e - 1 };
        let nm = m | idx[ux][uy];
        if ne > visit[ux][uy][nm as usize] {
          visit[ux][uy][nm as usize] = ne;
          q.push_back((nx, ny, ne, nm, s + 1));
        }
      }
    }
    -1
  }
}
