impl Solution {
  pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
    let mut total = apple.iter().sum::<i32>();
    let mut capacity = capacity;
    capacity.sort_unstable();
    capacity.reverse();

    for i in 0..capacity.len() {
      total -= capacity[i];
      if total <= 0 {
        return i as i32 + 1;
      }
    }
    capacity.len() as _
  }
}
