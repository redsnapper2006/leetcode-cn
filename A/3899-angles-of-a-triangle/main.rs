impl Solution {
  pub fn internal_angles(sides: Vec<i32>) -> Vec<f64> {
    let mut sides = sides;
    sides.sort_unstable();
    if sides[0] + sides[1] <= sides[2] {
      return vec![];
    }

    let a = sides[0];
    let b = sides[1];
    let c = sides[2];
    vec![
      ((b * b + c * c - a * a) as f64 / (2 * b * c) as f64).acos().to_degrees(),
      ((a * a + c * c - b * b) as f64 / (2 * a * c) as f64).acos().to_degrees(),
      ((a * a + b * b - c * c) as f64 / (2 * a * b) as f64).acos().to_degrees(),
    ]
  }
}
