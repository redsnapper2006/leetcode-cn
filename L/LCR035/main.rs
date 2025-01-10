struct Solution {}

impl Solution {
  pub fn find_min_difference(time_points: Vec<String>) -> i32 {
    let mut tp = time_points;
    tp.sort_unstable();
    let mut ans: i32 = 24 * 60 + 1;
    (1..tp.len()).for_each(|idx| {
      let m1 =
        tp[idx - 1][0..2].parse::<i32>().unwrap() * 60 + tp[idx - 1][3..].parse::<i32>().unwrap();
      let m2 = tp[idx][0..2].parse::<i32>().unwrap() * 60 + tp[idx][3..].parse::<i32>().unwrap();
      ans = ans.min(m2 - m1).min(24 * 60 - m2 + m1);
    });
    let m1 = tp[tp.len() - 1][0..2].parse::<i32>().unwrap() * 60
      + tp[tp.len() - 1][3..].parse::<i32>().unwrap();
    let m2 = tp[0][0..2].parse::<i32>().unwrap() * 60 + tp[0][3..].parse::<i32>().unwrap();
    ans.min(24 * 60 - m1 + m2)
  }
}

fn main() {
  println!(
    "{}",
    Solution::find_min_difference(vec!["23:59".to_string(), "23:59".to_string()])
  );
}
