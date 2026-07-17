impl Solution {
  pub fn seconds_between_times(start_time: String, end_time: String) -> i32 {
    (end_time[0..2].parse::<i32>().unwrap() - start_time[0..2].parse::<i32>().unwrap()) * 60 * 60
      + (end_time[3..5].parse::<i32>().unwrap() - start_time[3..5].parse::<i32>().unwrap()) * 60
      + (end_time[6..].parse::<i32>().unwrap() - start_time[6..].parse::<i32>().unwrap())
  }
}
