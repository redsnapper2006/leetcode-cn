use std::collections::BinaryHeap;

impl Solution {
  pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
    let mut avail: BinaryHeap<(i32, i32)> =
      servers.iter().enumerate().map(|(i, &p)| (-p, i as i32 * -1)).collect();
    let mut pending: BinaryHeap<(i32, i32, i32)> = BinaryHeap::new();

    let mut ans: Vec<i32> = vec![-1; tasks.len()];
    let mut time = 0i32;
    for idx in 0..tasks.len() {
      time = time.max(idx as i32);
      while pending.len() > 0 && pending.peek().unwrap().0 >= -time {
        let (_, p, i) = pending.pop().unwrap();
        avail.push((p, i));
      }

      let (p, i) = avail.pop().unwrap_or_else(|| {
        let (t, p, i) = pending.pop().unwrap();
        time = -t;
        (p, i)
      });

      ans[idx] = -i;
      pending.push(((time + tasks[idx]) * -1, p, i));
    }
    ans
  }
}
