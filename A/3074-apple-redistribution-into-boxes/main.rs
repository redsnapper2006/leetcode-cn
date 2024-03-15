impl Solution {
  pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
    let total = apple.iter().sum();
    let mut capacity = capacity;
    capacity.sort();

    let mut cnt: i32 = 0;

    let mut sum: i32 = 0;
    let mut ret: i32 = 0;
    capacity.iter().rev().for_each(|&v| {
      sum += v;
      cnt += 1;
      if ret == 0 && sum >= total {
        ret = cnt;
      }
    });
    ret
  }
}
