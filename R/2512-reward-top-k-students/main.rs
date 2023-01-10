struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn top_students(
    positive_feedback: Vec<String>,
    negative_feedback: Vec<String>,
    report: Vec<String>,
    student_id: Vec<i32>,
    k: i32,
  ) -> Vec<i32> {
    let mut buf: Vec<(i32, i32)> = Vec::new();
    let mut score_map: HashMap<String, i32> = HashMap::new();
    for p in positive_feedback {
      score_map.insert(p, 3);
    }
    for n in negative_feedback {
      score_map.insert(n, -1);
    }
    for (i, r) in report.iter().enumerate() {
      let mut score: i32 = 0;
      for w in r.split(' ').collect::<Vec<&str>>() {
        let mut s = score_map.entry(w.to_string()).or_insert(0);
        score += *s;
      }
      buf.push((student_id[i], score));
    }
    buf.sort_by(|a, b| {
      if a.1 != b.1 {
        return b.1.cmp(&a.1);
      }
      a.0.cmp(&b.0)
    });
    buf.resize(k as usize, (-1, -1));
    buf.iter().map(|v| v.0).collect::<Vec<i32>>()
  }
}
