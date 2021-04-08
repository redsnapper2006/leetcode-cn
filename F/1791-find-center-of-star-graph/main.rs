struct Solution {}

impl Solution {
  pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    if edges[0][0] == edges[1][0] || edges[0][0] == edges[1][1] {
      return edges[0][0];
    }
    edges[0][1]
  }
  pub fn find_centerv2(edges: Vec<Vec<i32>>) -> i32 {
    let mut buf: Vec<i32> = vec![0; edges.len() + 1];
    for i in 0..edges.len() {
      buf[edges[i as usize][0] as usize - 1] += 1;
      buf[edges[i as usize][1] as usize - 1] += 1;
    }
    for i in 0..edges.len() + 1 {
      if buf[i as usize] as usize == edges.len() {
        return i as i32 + 1;
      }
    }
    -1
  }
}

fn main() {
  println!("{}", Solution::find_center(vec![vec![0; 2]; 5]));
}
