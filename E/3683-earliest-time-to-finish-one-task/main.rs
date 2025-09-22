impl Solution {
  pub fn earliest_time(tasks: Vec<Vec<i32>>) -> i32 {
    tasks.iter().fold(201, |mn, task| mn.min(task[0] + task[1]))
  }
}
