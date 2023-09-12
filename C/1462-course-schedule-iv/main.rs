struct Solution {}

use std::collections::{HashMap, HashSet};
impl Solution {
  pub fn check_if_prerequisite(
    num_courses: i32,
    prerequisites: Vec<Vec<i32>>,
    queries: Vec<Vec<i32>>,
  ) -> Vec<bool> {
    let mut ingress: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut outgress: HashMap<i32, HashSet<i32>> = HashMap::new();

    for prerequisite in prerequisites.iter() {
      let (a, b) = (prerequisite[0], prerequisite[1]);
      ingress.entry(b).or_insert(HashSet::new()).insert(a);
      outgress.entry(a).or_insert(HashSet::new()).insert(b);
    }

    let mut depend: HashMap<i32, HashSet<i32>> = HashMap::new();

    loop {
      let mut is_found: bool = false;
      for i in 0..num_courses {
        if !outgress.contains_key(&i) && ingress.contains_key(&i) {
          let mut dep1 = HashSet::new();
          if depend.contains_key(&i) {
            dep1 = depend.get(&i).unwrap().clone();
          }

          let parent = ingress.get(&i).unwrap();
          for p in parent {
            let mut dep = dep1.clone();
            dep.insert(i);

            if depend.contains_key(p) {
              let dep2 = depend.get(p).unwrap();
              dep = dep.union(dep2).cloned().collect();
            }
            depend.insert(*p, dep);

            let out = outgress.get_mut(p).unwrap();
            out.remove(&i);
            if out.len() == 0 {
              outgress.remove(p);
            }
          }
          ingress.remove(&i);

          is_found = true;
          break;
        }
      }

      if !is_found {
        break;
      }
    }

    let mut result: Vec<bool> = Vec::new();
    queries.iter().for_each(|q| {
      let (a, b) = (q[0], q[1]);
      if depend.contains_key(&a) && depend.get(&a).unwrap().contains(&b) {
        result.push(true);
      } else {
        result.push(false);
      }
    });
    result
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::check_if_prerequisite(
      4,
      vec![vec![2, 3], vec![2, 1], vec![0, 3], vec![0, 1]],
      vec![
        vec![0, 1],
        vec![0, 3],
        vec![2, 3],
        vec![3, 0],
        vec![2, 0],
        vec![0, 2]
      ]
    )
  );
}
