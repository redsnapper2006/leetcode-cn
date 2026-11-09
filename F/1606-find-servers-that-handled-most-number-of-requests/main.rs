use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
  pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
    let k_usize = k as usize;
    let k_i32 = k;
    let mut available: BinaryHeap<Reverse<i32>> = (0..k_usize).map(|i| Reverse(i as i32)).collect();

    let mut busy: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();

    let mut requests = vec![0i32; k_usize];
    let mut max_request = 0i32;
    let mut ans: Vec<i32> = Vec::new();
    for (i, &t) in arrival.iter().enumerate() {
      let i32 = i as i32;

      while let Some(&Reverse((end_t, id))) = busy.peek() {
        if end_t > t {
          break;
        }
        busy.pop();
        let pseudo = i32 + (id - i32).rem_euclid(k_i32);
        available.push(Reverse(pseudo));
      }

      if available.is_empty() {
        continue;
      }

      let Reverse(pseudo) = available.pop().unwrap();
      let id = pseudo.rem_euclid(k_i32);

      requests[id as usize] += 1;

      if requests[id as usize] > max_request {
        max_request = requests[id as usize];
        ans = vec![id];
      } else if requests[id as usize] == max_request {
        ans.push(id);
      }

      busy.push(Reverse((t + load[i], id)));
    }

    ans
  }
}
