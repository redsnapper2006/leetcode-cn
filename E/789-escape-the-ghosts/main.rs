struct Solution {}

impl Solution {
  pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
    let mut x = target[0];
    if x < 0 {
      x = -x;
    }
    let mut y = target[1];
    if y < 0 {
      y = -y;
    }
    let distance = x + y;
    for g in &ghosts {
      let mut xg = g[0] - target[0];
      if xg < 0 {
        xg = -xg;
      }
      let mut yg = g[1] - target[1];
      if yg < 0 {
        yg = -yg;
      }
      if xg + yg <= distance {
        return false;
      }
    }
    true
  }
}

fn main() {
  println!(
    "{}",
    Solution::escape_ghosts(vec![vec![0; 5]; 5], vec![0; 2])
  );
}
