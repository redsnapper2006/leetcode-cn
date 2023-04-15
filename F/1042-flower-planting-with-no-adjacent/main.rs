struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![0; n as usize];

    let mut edge: HashMap<i32, Vec<i32>> = HashMap::new();

    paths.iter().for_each(|path| {
      let e1 = path[0] - 1;
      let e2 = path[1] - 1;

      let mut e11 = edge.entry(e1).or_insert(Vec::new());
      e11.push(e2);
      let mut e22 = edge.entry(e2).or_insert(Vec::new());
      e22.push(e1);
    });

    (0..n).for_each(|idx| {
      let mut colored: [bool; 5] = [false; 5];
      let next = edge.entry(idx).or_insert(Vec::new());
      next.iter().for_each(|&n| {
        colored[ret[n as usize] as usize] = true;
      });
      let mut ci: i32 = 1;
      while ci <= 4 {
        if !colored[ci as usize] {
          ret[idx as usize] = ci;
          break;
        }
        ci += 1;
      }
    });

    ret
  }
}
