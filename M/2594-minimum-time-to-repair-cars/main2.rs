struct Solution {}

impl Solution {
  pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
    let mut start: i64 = 1;
    let mut end: i64 = ranks[0] as i64 * cars as i64 * cars as i64;
    while start < end {
      let m = start + (end - start) / 2;
      let mut sum: i64 = 0;
      ranks.iter().for_each(|&v| {
        sum += ((m / v as i64) as f64).sqrt() as i64;
      });
      if sum >= cars as i64 {
        end = m;
      } else {
        start = m + 1;
      }
    }
    start
  }
}

fn main() {
  println!("{}", Solution::repair_cars(vec![4, 2, 3, 1], 10));
  println!("{}", Solution::repair_cars(vec![5, 1, 8], 6));
}
