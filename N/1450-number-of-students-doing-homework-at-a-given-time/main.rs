struct Solution {}

impl Solution {
  pub fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    (0..start_time.len())
      .map(|idx| {
        if start_time[idx] <= query_time && end_time[idx] >= query_time {
          1
        } else {
          0
        }
      })
      .sum::<i32>()
  }
}
