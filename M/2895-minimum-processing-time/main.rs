impl Solution {
  pub fn min_processing_time(processor_time: Vec<i32>, tasks: Vec<i32>) -> i32 {
    let mut tasks = tasks;
    tasks.sort_unstable();
    let mut processor_time = processor_time;
    processor_time.sort_unstable();

    let mut ans: i32 = 0;
    for i in processor_time.len() {
      ans = ans.max(processor_time[i] + tasks[tasks.len() - 1 - i * 4]);
    }

    ans
  }
}
