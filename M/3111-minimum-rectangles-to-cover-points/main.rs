struct Solution {}

impl Solution {
  pub fn min_rectangles_to_cover_points(points: Vec<Vec<i32>>, w: i32) -> i32 {
    let mut points = points;
    points.sort_unstable_by(|x, y| x[0].cmp(&y[0]));

    let mut prev: i32 = -1;
    let mut cnt: i32 = 0;
    points.iter().for_each(|p| {
      if prev < p[0] {
        cnt += 1;
        prev = p[0] + w;
      }
    });
    cnt
  }
}

fn main() {
  println!(
    "{}",
    Solution::min_rectangles_to_cover_points(
      vec![
        vec![2, 1],
        vec![1, 0],
        vec![1, 4],
        vec![1, 8],
        vec![3, 5],
        vec![4, 6]
      ],
      1
    )
  );

  println!(
    "{}",
    Solution::min_rectangles_to_cover_points(
      vec![
        vec![0, 0],
        vec![1, 1],
        vec![2, 2],
        vec![3, 3],
        vec![4, 4],
        vec![5, 5],
        vec![6, 6]
      ],
      2
    )
  );
  println!(
    "{}",
    Solution::min_rectangles_to_cover_points(vec![vec![2, 3], vec![1, 2]], 0)
  );
  println!(
    "{}",
    Solution::min_rectangles_to_cover_points(vec![vec![0, 0]], 1000000000)
  );
}
