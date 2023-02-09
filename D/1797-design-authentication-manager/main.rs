use std::collections::HashMap;
struct AuthenticationManager {
  t_t_l: i32,
  t_m: HashMap<String, i32>,
}

impl AuthenticationManager {
  fn new(timeToLive: i32) -> Self {
    AuthenticationManager {
      t_m: HashMap::new(),
      t_t_l: timeToLive,
    }
  }

  fn generate(&mut self, token_id: String, current_time: i32) {
    let mut v = self.t_m.entry(token_id).or_insert(0);
    *v = current_time;
  }

  fn renew(&mut self, token_id: String, current_time: i32) {
    if !self.t_m.contains_key(&token_id) {
      return;
    }
    let mut v = self.t_m.get_mut(&token_id).unwrap();
    if *v + self.t_t_l <= current_time {
      self.t_m.remove(&token_id);
      return;
    }
    *v = current_time;
  }

  fn count_unexpired_tokens(&mut self, current_time: i32) -> i32 {
    let ttl = self.t_t_l;
    self.t_m.retain(|k, v| *v + ttl > current_time);
    self.t_m.len() as i32
  }
}
