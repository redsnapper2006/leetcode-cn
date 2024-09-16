struct Solution {}
impl Solution {
  pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let sum = distance.iter().sum::<i32>();
    let s = start.min(destination) as usize;
    let d = start.max(destination) as usize;
    let v = (s..d).fold(0, |sum, idx| sum + distance[idx]);
    v.min(sum - v)
  }
}
