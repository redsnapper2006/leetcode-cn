struct ExamTracker {
  t: Vec<(i32, i64)>,
}

impl ExamTracker {
  fn new() -> Self {
    ExamTracker { t: vec![(0, 0)] }
  }

  fn record(&mut self, time: i32, score: i32) {
    let prev = self.t.last().unwrap_or(&(0, 0)).1;
    self.t.push((time, prev + score as i64));
  }

  fn total_score(&self, start_time: i32, end_time: i32) -> i64 {
    let start = match self.t.binary_search_by(|probe| probe.0.cmp(&start_time)) {
      Ok(ov) => ov - 1,
      Err(ev) => ev - 1,
    };
    let end = match self.t.binary_search_by(|probe| probe.0.cmp(&end_time)) {
      Ok(ov) => ov,
      Err(ev) => ev - 1,
    };
    self.t[end].1 - self.t[start].1
  }
}
