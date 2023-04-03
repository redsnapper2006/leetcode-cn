struct Solution {}

impl Solution {
  pub fn convert_time(current: String, correct: String) -> i32 {
    let cur_arr: Vec<&str> = current.split(':').collect();
    let cur_hours: i32 = cur_arr[0].parse::<i32>().unwrap_or(0);
    let cur_minutes: i32 = cur_arr[1].parse::<i32>().unwrap_or(0);

    let cor_arr: Vec<&str> = correct.split(':').collect();

    let cor_hours: i32 = cor_arr[0].parse::<i32>().unwrap_or(0);
    let cor_minutes: i32 = cor_arr[1].parse::<i32>().unwrap_or(0);

    let mut hours_diff = cor_hours - cur_hours;
    let mut minutes_diff = cor_minutes - cur_minutes;
    if minutes_diff < 0 {
      hours_diff -= 1;
      minutes_diff += 60;
    }
    let mut diff = hours_diff * 60 + minutes_diff;
    let mut times: i32 = 0;
    while diff > 0 {
      if diff >= 60 {
        times += diff / 60;
        diff %= 60;
      } else if diff >= 15 {
        times += diff / 15;
        diff %= 15;
      } else if diff >= 5 {
        times += diff / 5;
        diff %= 5;
      } else {
        times += diff;
        diff -= diff;
      }
    }
    times
  }
}
