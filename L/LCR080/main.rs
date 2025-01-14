struct Solution {}

impl Solution {
  pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut buf: Vec<Vec<i32>> = vec![vec![]];
    for i in 1..=n {
      let size: usize = buf.len();
      for j in 0..size {
        if buf[j].len() >= (k as usize) {
          continue;
        }
        let mut t = buf[j].clone();
        t.push(i);
        buf.push(t);
      }
    }
    buf
      .into_iter()
      .filter(|x| x.len() == (k as usize))
      .collect::<Vec<Vec<i32>>>()
  }
}

fn main() {
  println!("{:?}", Solution::combine(4, 2));
}
