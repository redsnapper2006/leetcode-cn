use std::collections::{HashMap, VecDeque};

impl Solution {
  pub fn min_jumps(arr: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, Vec<usize>> = HashMap::new();
    arr.iter().enumerate().for_each(|(idx, &v)| {
      m.entry(v).or_insert(vec![]).push(idx);
    });

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back((0, 0));

    let mut visited: Vec<bool> = vec![false; arr.len()];
    visited[0] = true;

    while q.len() > 0 {
      let (cur, step) = q.pop_front().unwrap();
      if cur == arr.len() as i32 - 1 {
        return step;
      }

      if m.contains_key(&arr[cur as usize]) {
        for &n in m.get(&arr[cur as usize]).unwrap() {
          if !visited[n] {
            visited[n] = true;
            q.push_back((n as i32, step + 1));
          }
        }
      }
      m.remove(&arr[cur as usize]);

      if cur > 0 && !visited[cur as usize - 1] {
        visited[cur as usize - 1] = true;
        q.push_back((cur - 1, step + 1));
      }
      if cur < arr.len() as i32 - 1 && !visited[cur as usize + 1] {
        visited[cur as usize + 1] = true;
        q.push_back((cur + 1, step + 1));
      }
    }
    -1
  }
}
