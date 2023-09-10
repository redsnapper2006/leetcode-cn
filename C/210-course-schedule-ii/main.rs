struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ingress: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut out: HashMap<i32, Vec<i32>> = HashMap::new();

    (0..num_courses).for_each(|i| {
      ingress.insert(i, Vec::new());
      out.insert(i, Vec::new());
    });

    prerequisites.iter().for_each(|p| {
      ingress.get_mut(&p[0]).unwrap().push(p[1]);
      out.get_mut(&p[1]).unwrap().push(p[0]);
    });

    let mut result: Vec<i32> = Vec::new();

    loop {
      let mut found = false;
      let mut candi: i32 = -1;

      for (k, v) in ingress.iter() {
        if v.len() == 0 {
          found = true;
          result.push(*k);
          candi = *k;
          break;
        }
      }
      if !found {
        break;
      }
      ingress.remove(&candi);
      for i in out.get(&candi).unwrap() {
        ingress.get_mut(i).unwrap().retain(|x| *x != candi);
      }
    }

    if ingress.len() > 0 {
      return Vec::new();
    }
    result
  }
}
