use std::collections::VecDeque;

impl Solution {
  pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    let mut arr = arr;

    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(start);

    while q.len() > 0 {
      let nxt = q.pop_front().unwrap() as usize;
      if arr[nxt] == 0 {
        return true;
      }

      if nxt as i32 - arr[nxt] >= 0 && arr[nxt - arr[nxt] as usize] != -1 {
        q.push_back(nxt as i32 - arr[nxt]);
      }
      if nxt as i32 + arr[nxt] < arr.len() as i32 && arr[nxt + arr[nxt] as usize] != -1 {
        q.push_back(nxt as i32 + arr[nxt]);
      }

      arr[nxt] = -1;
    }
    false
  }
}
