struct Solution {}

impl Solution {
  pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut max = 0;
    for i in &trips {
      if max < i[2] {
        max = i[2];
      }
    }
    let mut buf: Vec<i32> = vec![0; max as usize + 1];
    for i in &trips {
      buf[i[1] as usize] += i[0];
      buf[i[2] as usize] -= i[0];
    }
    let mut sum = 0;
    for i in 0..max {
      sum += buf[i as usize];
      if sum > capacity {
        return false;
      }
    }
    true
  }
}

fn main() {
  println!("{}", Solution::car_pooling(vec![vec![0; 5]; 5], 3));
}
