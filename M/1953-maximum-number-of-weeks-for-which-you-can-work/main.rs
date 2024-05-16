impl Solution {
  pub fn number_of_weeks(milestones: Vec<i32>) -> i64 {
    let mut milestones = milestones.iter().map(|&x| x as i64).collect::<Vec<i64>>();
    let mut max = milestones[0];
    let mut total : i64 = 0;
    milestones.iter().for_each(|&v| {
      max = max.max(v);
      total +=v;
    });

    match max > total - max + 1 {
      true => (total - max) * 2 + 1,
      false => total,
    }
  }
}
