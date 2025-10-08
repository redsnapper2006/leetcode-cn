impl Solution {
  pub fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
  ) -> i32 {
    let mut l_min: i32 = land_start_time[0] + land_duration[0];
    for i in 1..land_start_time.len() {
      l_min = l_min.min(land_start_time[i] + land_duration[i]);
    }
    let mut w_min: i32 = water_start_time[0] + water_duration[0];
    for i in 1..water_start_time.len() {
      w_min = w_min.min(water_start_time[i] + water_duration[i]);
    }

    let mut ans: i32 = 3001;
    for i in 0..water_start_time.len() {
      ans = ans.min(l_min.max(water_start_time[i]) + water_duration[i]);
    }
    for i in 0..land_start_time.len() {
      ans = ans.min(w_min.max(land_start_time[i]) + land_duration[i]);
    }

    ans
  }
}
