use std::collections::HashSet;

impl Solution {
  pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
    let mut buf: Vec<i32> = vec![0];

    for i in 0..mat.len() {
      let mut t: Vec<i32> = vec![];
      let mut set: HashSet<i32> = HashSet::new();
      for j in 0..mat[i].len() {
        for bi in 0..buf.len() {
          let v = mat[i][j] + buf[bi];
          if set.contains(&v) {
            continue;
          }
          t.push(v);
          set.insert(v);
        }
      }
      buf = t;
    }
    buf.sort_unstable();
    let ll = match buf.binary_search(&target) {
      Ok(ov) => ov,
      Err(ev) => ev,
    };
    println!("{:?} {}", buf, ll);

    if ll == buf.len() {
      target - buf[buf.len() - 1]
    } else if ll == 0 {
      buf[ll] - target
    } else {
      (buf[ll] - target).min(target - buf[ll - 1])
    }
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::minimize_the_difference(
      vec![
        vec![10, 3, 7, 7, 9, 6, 9, 8, 9, 5],
        vec![1, 1, 6, 8, 6, 7, 7, 9, 3, 9],
        vec![3, 4, 4, 1, 3, 6, 3, 3, 9, 9],
        vec![6, 9, 9, 3, 8, 7, 9, 6, 10, 6]
      ],
      5
    )
  );
}
