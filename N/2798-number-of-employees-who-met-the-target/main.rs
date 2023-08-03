impl Solution {
  pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
    hours
      .into_iter()
      .filter(|&v| v >= target)
      .collect::<Vec<i32>>()
      .len() as i32
  }
}
