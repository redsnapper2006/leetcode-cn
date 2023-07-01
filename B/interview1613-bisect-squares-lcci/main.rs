struct Solution {}

impl Solution {
  pub fn cut_squares(square1: Vec<i32>, square2: Vec<i32>) -> Vec<f64> {
    fn center(square: &Vec<i32>) -> (f64, f64) {
      (
        square[0] as f64 + (square[2] as f64) / 2.0,
        square[1] as f64 + (square[2] as f64) / 2.0,
      )
    }
    fn calc(square: Vec<i32>, k: f64, cx: f64, cy: f64) -> (f64, f64, f64, f64) {
      let mut x11: f64 = (square[1] as f64 - cy) / k + cx;
      let mut x12: f64 = (square[1] as f64 + square[2] as f64 - cy) / k + cx;
      let mut y11: f64 = square[1] as f64;
      let mut y12: f64 = square[1] as f64 + square[2] as f64;

      if (square[0] as f64 - cx) * k + cy >= square[1] as f64
        && (square[0] as f64 - cx) * k + cy <= square[1] as f64 + square[2] as f64
      {
        x11 = square[0] as f64;
        y11 = (square[0] as f64 - cx) * k + cy;
        x12 = square[0] as f64 + square[2] as f64;
        y12 = (square[0] as f64 + square[2] as f64 - cx) * k + cy;
      }
      (x11, y11, x12, y12)
    }
    let (c1x, c1y) = center(&square1);
    let (c2x, c2y) = center(&square2);

    match c1x == c2x {
      true => {
        vec![
          c1x,
          (c1y - (square1[2] as f64) / 2.0).min(c2y - (square2[2] as f64) / 2.0),
          c1x,
          (c1y + (square1[2] as f64) / 2.0).max(c2y + (square2[2] as f64) / 2.0),
        ]
      }
      false => {
        let k: f64 = (c2y - c1y) / (c2x - c1x);

        let (x11, y11, x12, y12) = calc(square1, k, c1x, c1y);
        let (x21, y21, x22, y22) = calc(square2, k, c2x, c2y);

        let mut buf: Vec<(f64, f64)> = vec![(x11, y11), (x12, y12), (x21, y21), (x22, y22)];
        buf.sort_by(|p1, p2| {
          if &p1.0 == &p2.0 {
            return p1.1.partial_cmp(&p2.1).unwrap();
          }
          p1.0.partial_cmp(&p2.0).unwrap()
        });

        vec![buf[0].0, buf[0].1, buf[3].0, buf[3].1]
      }
    }
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::cut_squares(vec![-1, -1, 2], vec![0, -1, 2])
  );

  println!(
    "{:?}",
    Solution::cut_squares(vec![249, -199, 5], vec![-1, 136, 76])
  );
}
