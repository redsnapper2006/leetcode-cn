impl Solution {
  pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    points.sort_by(|p1, p2| {
      if p1[0] != p2[0] {
        return p2[0].cmp(&p1[0]);
      }
      p1[1].cmp(&p2[1])
    });

    let mut ans: i32 = 0;
    for i in 0..points.len() {
      let base_y = points[i][1];
      let mut ref_y: i32 = -1;
      for j in i + 1..points.len() {
        if points[j][1] < base_y {
          continue;
        }
        if ref_y == -1 || points[j][1] < ref_y {
          ref_y = points[j][1];
          ans += 1;
        }
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::number_of_pairs(vec![vec![6, 2], vec![4, 4], vec![2, 6]])
  );
}
