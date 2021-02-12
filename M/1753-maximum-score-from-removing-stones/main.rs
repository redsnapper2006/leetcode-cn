struct Solution {}

impl Solution {
  pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
    let mut buf : Vec<i32> = vec![a,b,c];
    buf.sort();
    if buf[0]+buf[1] <=buf[2] {
      return buf[0]+buf[1];
    }
    let m = ( buf[0]+buf[1] -buf[2])/2;
    m+buf[2]
  }
}

fn main() {
  println!("{}", Solution::maximum_score(1, 2, 3));
}
