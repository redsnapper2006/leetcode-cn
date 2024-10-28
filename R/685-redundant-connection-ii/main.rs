struct Solution {}

impl Solution {
  pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let n = edges.len();
    let mut parent: Vec<i32> = vec![0; n + 1];
    let mut ancestor: Vec<i32> = vec![0; n + 1];
    (0..=n).for_each(|idx| {
      parent[idx] = idx as i32;
      ancestor[idx] = idx as i32;
    });

    fn find(idx: i32, ancestor: &mut Vec<i32>) -> i32 {
      if ancestor[idx as usize] != idx {
        let t = find(ancestor[idx as usize], ancestor);
        ancestor[idx as usize] = t;
      }
      ancestor[idx as usize]
    }
    fn union(from: i32, to: i32, ancestor: &mut Vec<i32>) {
      let t = find(to, ancestor);
      let f = find(from, ancestor);
      ancestor[f as usize] = t;
    }

    let mut cycle: Vec<i32> = Vec::new();
    let mut conflict: Vec<i32> = Vec::new();
    edges.into_iter().for_each(|edge| {
      let from = edge[0];
      let to = edge[1];

      if parent[to as usize] != to {
        conflict = edge;
        return;
      }

      parent[to as usize] = from;
      let t = find(to, &mut ancestor) ;
      let f =  find(from, &mut ancestor);
      if f == t {
        cycle = edge;
        return;
      }
      union(from, to, &mut ancestor);
    });

    if conflict.len() == 0 {
      return cycle;
    }
    if cycle.len() != 0 {
      vec![parent[conflict[1] as usize], conflict[1]]
    } else {
      conflict
    }
  }
}
