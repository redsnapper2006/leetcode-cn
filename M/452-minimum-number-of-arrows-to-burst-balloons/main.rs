impl Solution {
  pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;

    points.sort_by(|x, y| {
      let x0 = &x[0];
      let y0 = &y[0];
      let x1 = &x[1];
      let y1 = &y[1];
      if x1 != y1 {
        return x1.cmp(y1);
      }
      y0.cmp(x0)
    });


    let mut base = points[0][1];
    let mut ans: i32 = 1;
    points.iter().for_each(|pt| {
      if pt[0] <= base && pt[1] >= base {
        return;
      }
      base = pt[1];
      ans += 1;
    });
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::find_min_arrow_shots(vec![
      vec![3, 9],
      vec![7, 12],
      vec![3, 8],
      vec![6, 8],
      vec![9, 10],
      vec![2, 9],
      vec![0, 9],
      vec![3, 9],
      vec![0, 6],
      vec![2, 8]
    ])
  );
}
