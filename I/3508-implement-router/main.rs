use std::collections::{HashMap,HashSet,VecDeque};

struct Router {
  count: i32,
  limit: i32,
  que: VecDeque<(i32, i32, i32)>,
  key: HashSet<(i32, i32, i32)>,
  d_que: HashMap<i32, VecDeque<i32>>,
}

impl Router {
  fn new(memory_limit: i32) -> Self {
    Router {
      count: 0,
      limit: memory_limit,
      que: VecDeque::new(),
      key: HashSet::new(),
      d_que: HashMap::new(),
    }
  }

  fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
    if self.key.contains(&(source, destination, timestamp)) {
      return false;
    }

    if self.count >= self.limit {
      let top = self.que.pop_front().unwrap();
      self.d_que.get_mut(&top.1).unwrap().pop_front();
      self.count -= 1;
      self.key.remove(&top);
    }

    self.count += 1;
    self.que.push_back((source, destination, timestamp));
    self.key.insert((source, destination, timestamp));
    self
      .d_que
      .entry(destination)
      .and_modify(|x| x.push_back(timestamp))
      .or_insert(VecDeque::from([timestamp]));
    true
  }

  fn forward_packet(&mut self) -> Vec<i32> {
    if self.que.len() == 0 {
      return vec![];
    }

    let top = self.que.pop_front().unwrap();
    self.d_que.get_mut(&top.1).unwrap().pop_front();
    self.count -= 1;
    self.key.remove(&top);
    vec![top.0, top.1, top.2]
  }

  fn get_count(&mut self, destination: i32, start_time: i32, end_time: i32) -> i32 {
    if !self.d_que.contains_key(&destination) {
      return 0;
    }
    let seq = self.d_que.get_mut(&destination).unwrap();

    let start = seq.partition_point(|&x| x < start_time);
    let end = seq.partition_point(|&x| x <= end_time);

    (end - start) as i32
  }
}

fn main() {
  let mut router = Router::new(3);
  println!("{} ", router.add_packet(1, 4, 90));
  println!("{} ", router.add_packet(2, 5, 90));
  println!("{} ", router.add_packet(1, 4, 90));
  println!("{} ", router.add_packet(3, 5, 95));
  println!("{} ", router.add_packet(4, 5, 105));
  let f = router.forward_packet();
  println!("{} {} {}", f[0], f[1], f[2]);
  println!("{} ", router.add_packet(5, 2, 110));
  println!("{}", router.get_count(5, 100, 110));

  let mut router = Router::new(4);
  println!("{} ", router.add_packet(4, 2, 1));
  println!("{} ", router.add_packet(3, 2, 1));
  println!("{}", router.get_count(2, 1, 1));

  let mut router = Router::new(2);
  println!("{} ", router.add_packet(4, 3, 1));
  println!("{} ", router.add_packet(5, 4, 1));
  println!("{} ", router.add_packet(2, 3, 4));
  println!("{}", router.get_count(4, 1, 3));

  let mut router = Router::new(3);
  println!("{} ", router.add_packet(1, 4, 90));
  println!("{}", router.get_count(5, 100, 110));
}
