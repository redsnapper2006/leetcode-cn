impl Solution {
  pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
    let mut l = lower;
    let mut r = upper;
    differences.iter().for_each(|v| {
      l = (l + v).max(lower);
      r = (r + v).min(upper);
    });

    if r - l >= 0 {
      r - l + 1
    } else {
      0
    }
  }
}
