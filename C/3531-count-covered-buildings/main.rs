impl Solution {
  pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut x_m: Vec<(i32, i32)> = vec![(i32::MAX, -1); n + 1];
    let mut y_m: Vec<(i32, i32)> = vec![(i32::MAX, -1); n + 1];
    buildings.iter().for_each(|build| {
      x_m[build[0] as usize].0 = x_m[build[0] as usize].0.min(build[1]);
      x_m[build[0] as usize].1 = x_m[build[0] as usize].1.max(build[1]);

      y_m[build[1] as usize].0 = y_m[build[1] as usize].0.min(build[0]);
      y_m[build[1] as usize].1 = y_m[build[1] as usize].1.max(build[0]);
    });

    let mut ans: i32 = 0;
    buildings.iter().for_each(|build| {
      let x = build[0];
      let y = build[1];
      let (x1, x2) = y_m[y as usize];
      let (y1, y2) = x_m[x as usize];
      if x > x1 && x < x2 && y > y1 && y < y2 {
        ans += 1;
      }
    });

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::count_covered_buildings(
      3,
      vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]]
    )
  );
}
