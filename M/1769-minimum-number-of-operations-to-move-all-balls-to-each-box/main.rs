struct Solution {}

impl Solution {
  pub fn min_operations(boxes: String) -> Vec<i32> {
    let o = boxes.as_bytes();

    let mut os: Vec<i32> = vec![0; o.len()];
    let mut rs: Vec<i32> = vec![0; o.len()];
    let mut cnt: i32 = 0;
    let mut sum: i32 = 0;
    for i in 0..o.len() {
      os[i] = sum;
      if o[i] == '1' as u8 {
        cnt += 1;
      }
      sum += cnt;
    }
    cnt = 0;
    sum = 0;
    for i in (0..o.len()).rev() {
      rs[i] = sum;
      if o[i] == '1' as u8 {
        cnt += 1;
      }
      sum += cnt;
    }
    let mut ret: Vec<i32> = vec![0; o.len()];
    for i in 0..o.len() {
      ret[i] = os[i] + rs[i];
    }
    ret
  }
}

fn main() {
  println!("{:?}", Solution::min_operations(String::from("110")));
}
