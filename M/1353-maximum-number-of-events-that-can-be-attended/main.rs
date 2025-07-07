use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
    let mut events = events;
    events.sort_unstable();
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    let mut max_d: i32 = events.iter().map(|x| x[1]).max().unwrap();

    let mut ans: i32 = 0;
    let mut cur: i32 = -1;
    let mut idx: usize = 0;
    for i in 1..=max_d {
      while idx < events.len() && events[idx][0] <= i {
        heap.push(Reverse(events[idx][1]));
        idx += 1;
      }
      while heap.len() > 0 && heap.peek().unwrap().0 < i {
        heap.pop();
      }
      if heap.len() > 0 {
        heap.pop();
        ans += 1;
      }
    }

    ans
  }
}
