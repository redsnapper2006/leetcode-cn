use std::collections::HashMap;

impl Solution {
  pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    let mut m: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut m2: HashMap<i64, HashMap<String, i32>> = HashMap::new();

    (0..points.len()).for_each(|i| {
      let x = &points[i];
      (i + 1..points.len()).for_each(|j| {
        let y = &points[j];
        let mut k: String = (1e9 as i32).to_string();
        let mut b: f64 = x[0] as f64;
        if x[0] != y[0] {
          let mut kk = (y[1] - x[1]) as f64 / (y[0] - x[0]) as f64;
          if kk == -0.0 {
            kk = 0.0;
          }
          k = kk.to_string();
          b = (x[1] as i64 * (y[0] - x[0]) as i64 - x[0] as i64 * (y[1] - x[1]) as i64) as f64
            / (y[0] - x[0]) as f64;
          if b == -0.0 {
            b = 0.0;
          }
        }

        *m.entry(k.clone()).or_insert(HashMap::new()).entry(b.to_string()).or_insert(0) += 1;
        let mid = (x[0] + y[0]) as i64 * 1000000 + (x[1] + y[1]) as i64;
        *m2.entry(mid).or_insert(HashMap::new()).entry(k.to_string()).or_insert(0) += 1;
      });
    });

    let mut ans: i32 = 0;
    for (_, b) in &m {
      let mut sum: i32 = 0;
      for (_, c) in b {
        ans += c * sum;
        sum += c;
      }
    }
    for (_, b) in &m2 {
      let mut sum: i32 = 0;
      for (_, c) in b {
        ans -= c * sum;
        sum += c;
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::count_trapezoids(vec![
      vec![-3, 2],
      vec![3, 0],
      vec![2, 3],
      vec![3, 2],
      vec![2, -3]
    ])
  );

  println!(
    "{}",
    Solution::count_trapezoids(vec![vec![0, 0], vec![1, 0], vec![0, 1], vec![2, 1]])
  );
}
