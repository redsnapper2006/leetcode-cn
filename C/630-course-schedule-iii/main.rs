struct Solution {}

use std::collections::BinaryHeap;
impl Solution {
  pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
    let mut courses = courses;
    courses.sort_by(|a, b| a[1].cmp(&b[1]));

    let mut heap = BinaryHeap::new();
    let mut current: i32 = 0;
    courses.iter().for_each(|c| {
      if current + c[0] <= c[1] {
        current += c[0];
        heap.push(c[0]);
      } else if heap.len() > 0 && *heap.peek().unwrap() > c[0] {
        current += c[0] - heap.pop().unwrap();
        heap.push(c[0]);
      }
    });

    heap.len() as i32
  }
}
