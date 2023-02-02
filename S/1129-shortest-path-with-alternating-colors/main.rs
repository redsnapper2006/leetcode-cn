struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn shortest_alternating_paths(
    n: i32,
    red_edges: Vec<Vec<i32>>,
    blue_edges: Vec<Vec<i32>>,
  ) -> Vec<i32> {
    let col: Vec<Vec<i32>> = (0..2)
      .map(|idx| {
        let mut ret: Vec<i32> = vec![-1; n as usize];

        let mut red: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut blue: HashMap<i32, Vec<i32>> = HashMap::new();
        for r in &red_edges {
          let mut e = red.entry(r[0]).or_insert(Vec::new());
          e.push(r[1]);
        }
        for b in &blue_edges {
          let mut e = blue.entry(b[0]).or_insert(Vec::new());
          e.push(b[1]);
        }

        let mut edges = match idx {
          0 => vec![red, blue],
          _ => vec![blue, red],
        };

        let mut candi: Vec<i32> = vec![0];
        let mut steps: i32 = 0;
        while (candi.len() > 0) {
          let mut t: Vec<i32> = Vec::new();

          for c in candi {
            if ret[c as usize] == -1 {
              ret[c as usize] = steps;
            }
            if edges[(steps % 2) as usize].contains_key(&c) {
              for n in edges[(steps % 2) as usize].get(&c).unwrap() {
                t.push(*n);
              }
              edges[(steps % 2) as usize].remove(&c);
            }
          }
          candi = t;
          steps += 1;
        }
        ret
      })
      .collect::<Vec<Vec<i32>>>();

    let mut ret: Vec<i32> = vec![-1; n as usize];
    (0..n).for_each(|i| {
      let mut vr: i32 = col[0][i as usize];
      let mut vb: i32 = col[1][i as usize];
      if vr == -1 {
        ret[i as usize] = vb;
      } else {
        if vb == -1 {
          ret[i as usize] = vr;
        } else if vr > vb {
          ret[i as usize] = vb;
        } else {
          ret[i as usize] = vr;
        }
      }
    });
    ret
  }
}
