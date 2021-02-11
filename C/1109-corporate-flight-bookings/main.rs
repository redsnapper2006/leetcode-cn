struct Solution {}

impl Solution {
  pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut buf: Vec<i32> = vec![0; n as usize];
    for i in 0..bookings.len() {
      let b = &bookings[i];
      let res = b[2];
      buf[(b[0] - 1) as usize] += res;
      if b[1] < n {
        buf[b[1] as usize] -= res;
      }
    }
    let mut ret: Vec<i32> = Vec::new();
    let mut sum = 0;
    for i in 0..n {
      sum += buf[i as usize];
      ret.push(sum);
    }
    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::corp_flight_bookings(vec![vec![0; 5]; 5], 10)
  );
}
