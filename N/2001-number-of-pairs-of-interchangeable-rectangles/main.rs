use std::collections::HashMap;

impl Solution {
  pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
    let mut m: HashMap<String, i64> = HashMap::new();
    rectangles.iter().for_each(|&rect| {
      let f = format!("{:.10}", rect[0] as f64 / rect[1] as f64);
      m.entry(f).or_insert(0) += 1;
    });

    let mut ans: i64 = 0;
    for (_, v) in m {
      ans += v * (v - 1) / 2;
    }
    ans
  }
}
