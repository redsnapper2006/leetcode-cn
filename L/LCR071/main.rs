use rand::rngs::ThreadRng;
use rand::Rng;

struct Solution {
  s: Vec<i32>,
  t: i32,
  r: ThreadRng,
}

impl Solution {
  fn new(w: Vec<i32>) -> Self {
    let mut s: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    w.iter().for_each(|&v| {
      sum += v;
      s.push(sum);
    });
    Solution {
      s: s,
      t: sum,
      r: rand::thread_rng(),
    }
  }

  fn pick_index(&mut self) -> i32 {
    let v = self.r.gen_range(1, self.t + 1);
    (match self.s.binary_search(&v) {
      Ok(v) => v,
      Err(e) => e,
    }) as i32
  }
}
