impl Solution {
  pub fn elevator_requests(n: i32, requests: Vec<i32>) -> i32 {
    requests.iter().fold((0, 0), |(ans, prev), &v| (ans + (v - prev).abs(), v)).0
  }
}
