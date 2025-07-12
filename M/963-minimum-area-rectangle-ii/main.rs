impl Solution {
  pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
    let mut buf: Vec<Vec<(usize, i32, i32, bool, bool, i32)>> = vec![vec![]; points.len()];

    for i in 0..points.len() {
      for j in i + 1..points.len() {
        let dist = (points[i][0] - points[j][0]) * (points[i][0] - points[j][0])
          + (points[i][1] - points[j][1]) * (points[i][1] - points[j][1]);
        if points[i][0] == points[j][0] {
          buf[i].push((j, 0, i32::MAX, true, points[i][1] > points[j][1], dist));
          buf[j].push((i, 0, i32::MAX, true, points[i][1] < points[j][1], dist));
        } else {
          buf[i].push((
            j,
            (points[i][0] - points[j][0]),
            (points[i][1] - points[j][1]),
            points[i][0] > points[j][0],
            points[i][1] > points[j][1],
            dist,
          ));
          buf[j].push((
            i,
            (points[i][0] - points[j][0]),
            (points[i][1] - points[j][1]),
            points[i][0] < points[j][0],
            points[i][1] < points[j][1],
            dist,
          ));
        }
      }
    }

    for i in 0..buf.len() {
      buf[i].sort_unstable();
    }

    let mut ans: i64 = i64::MAX;
    for i in 0..buf.len() {
      for i1 in 0..buf[i].len() {
        let (n, x, y, x2x, y2y, dist1) = buf[i][i1];
        if n < i {
          continue;
        }
        let nk: (i32, i32) = match (x, y) {
          (0, _) => (i32::MAX, 0),
          (_, 0) => (0, i32::MAX),
          _ => (y, -x),
        };
        for j in 0..buf[n].len() {
          let (nn, xx, yy, _, _, dist2) = buf[n][j];
          if xx == 0 && nk.0 != 0 || yy == 0 && nk.1 != 0 || xx * nk.1 != yy * nk.0 {
            continue;
          }
          for m in 0..buf[nn].len() {
            let (_, xxx, yyy, xxx2xxx, yyy2yyy, dist3) = buf[nn][m];
            if (xxx == 0 && x == 0 || yyy == 0 && y == 0 || xxx * y == yyy * x)
              && yyy2yyy != y2y
              && dist3 == dist1
            {
              ans = ans.min(dist1 as i64 * dist2 as i64);
            }
          }
        }
      }
    }
    if ans == i64::MAX {
      0.0
    } else {
      (ans as f64).sqrt()
    }
  }
}
