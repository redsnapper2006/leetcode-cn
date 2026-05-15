impl Solution {
  pub fn score_validator(events: Vec<String>) -> Vec<i32> {
    let mut s: i32 = 0;
    let mut c: i32 = 0;
    for e in events {
      match e.as_str() {
        "W" => c += 1,
        "WD" | "NB" => s += 1,
        _ => s += e.parse::<i32>().unwrap(),
      }
      if c >= 10 {
        return vec![s, c];
      }
    }
    vec![s, c]
  }
}
