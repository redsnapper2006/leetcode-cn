impl Solution {
  pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
    let mut tasks = tasks;
    tasks.sort_by(|x, y| (x[1] - x[0]).cmp(&(y[1] - y[0])));

    tasks.iter().fold(0, |ans, task| (ans + task[0]).max(task[1]))
  }
}
