impl Solution {
  pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
    let mut minx: i32 = i32::MAX;
    let mut maxx: i32 = -1;
    let mut miny: i32 = i32::MAX;
    let mut maxy: i32 = -1;

    circles.iter().for_each(|cir| {
      minx = minx.min(cir[0] - cir[2]);
      maxx = maxx.max(cir[0] + cir[2]);
      miny = miny.min(cir[1] - cir[2]);
      maxy = maxy.max(cir[1] + cir[2]);
    });

    let mut cnt: i32 = 0;
    for x in minx..=maxx {
      for y in miny..=maxy {
        for c in &circles {
          if (x - c[0]) * (x - c[0]) + (y - c[1]) * (y - c[1]) <= c[2] * c[2] {
            cnt += 1;
            break;
          }
        }
      }
    }
    cnt
  }
}
