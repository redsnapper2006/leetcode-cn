impl Solution {
  pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    let mut start_time = start_time;
    let mut end_time = end_time;
    start_time.sort_unstable();
    end_time.sort_unstable();

    let mut interval: Vec<i32> = vec![];
    interval.push(start_time[0]);
    for i in 0..start_time.len() - 1 {
      interval.push(start_time[i + 1] - end_time[i]);
    }
    interval.push(event_time - end_time[end_time.len() - 1]);

    let mut ans: i32 = 0;
    let mut sum: i32 = 0;
    let k = k as usize + 1;
    for i in 0..interval.len() {
      sum += interval[i];
      if i >= k {
        sum -= interval[i - k];
      }
      ans = ans.max(sum);
    }
    ans
  }
}
