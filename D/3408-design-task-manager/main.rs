use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
struct Task {
  u: i32,
  t: i32,
  p: i32,
}

impl Ord for Task {
  fn cmp(&self, other: &Self) -> Ordering {
    self.p.cmp(&other.p).then_with(|| self.t.cmp(&other.t))
  }
}

impl PartialOrd for Task {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

struct TaskManager {
  m: HashMap<i32, (i32, i32, i32)>,
  bh: BinaryHeap<Task>,
}

impl TaskManager {
  fn new(tasks: Vec<Vec<i32>>) -> Self {
    let mut bh = BinaryHeap::new();
    let mut m = HashMap::new();
    for ts in tasks {
      m.insert(ts[1], (ts[0], ts[1], ts[2]));
      bh.push(Task {
        u: ts[0],
        t: ts[1],
        p: ts[2],
      });
    }
    TaskManager { m: m, bh: bh }
  }

  fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
    self.bh.push(Task {
      u: user_id,
      t: task_id,
      p: priority,
    });
    self.m.insert(task_id, (user_id, task_id, priority));
  }

  fn edit(&mut self, task_id: i32, new_priority: i32) {
    let (u, t, p) = *self.m.get(&task_id).unwrap();
    self.m.insert(task_id, (u, t, new_priority));
    self.bh.push(Task {
      u: u,
      t: t,
      p: new_priority,
    });
  }

  fn rmv(&mut self, task_id: i32) {
    self.m.remove(&task_id);
  }

  fn exec_top(&mut self) -> i32 {
    while self.bh.len() > 0 {
      let t = self.bh.pop().unwrap();
      if self.m.contains_key(&t.t)
        && self.m.get(&t.t).unwrap().2 == t.p
        && self.m.get(&t.t).unwrap().0 == t.u
      {
        self.m.remove(&t.t);
        return t.u;
      }
    }
    -1
  }
}
