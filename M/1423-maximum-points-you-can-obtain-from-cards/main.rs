struct Solution {}

impl Solution {
  pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
    let mut buf = vec![0; card_points.len()];
    for i in 0..card_points.len() {
      if i == 0 {
        buf[i] = card_points[i];
      } else {
        buf[i] = buf[i - 1] + card_points[i];
      }
    }

    let mut max = 0;
    if k == buf.len() as i32 {
      max = buf[buf.len() - 1];
    } else {
      max = buf[buf.len() - 1] - buf[buf.len() - k as usize - 1];
      for i in 0..k {
        if buf[buf.len() - 1] - buf[buf.len() - k as usize + i as usize] + buf[i as usize] > max {
          max = buf[buf.len() - 1] - buf[buf.len() - k as usize + i as usize] + buf[i as usize];
        }
      }
    }
    max
  }
}

fn main() {
  println!("{}", Solution::max_score(vec![0; 5], 3));
}
