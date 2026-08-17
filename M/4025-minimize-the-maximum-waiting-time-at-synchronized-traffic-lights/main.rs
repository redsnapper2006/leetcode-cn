impl Solution {
  pub fn min_penalty(period: i32, lights: Vec<i32>, arrival_time: Vec<i32>) -> i32 {
    let mx = *lights.iter().max().unwrap();
    let mut ans: i32 = 0;
    for a in &arrival_time {
      let a = a % period;
      if a >= mx {
        ans = ans.max(period - a);
      }
    }
    ans
  }
}
