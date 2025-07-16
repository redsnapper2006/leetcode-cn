use std::collections::HashSet;
impl Solution {
  pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let nc = num_courses as usize;
    let mut deps: Vec<HashSet<usize>> = vec![HashSet::new(); nc];
    let mut cnt: Vec<i32> = vec![0; nc];
    prerequisites.iter().for_each(|pre| {
      cnt[pre[0] as usize] += 1;
      deps[pre[1] as usize].insert(pre[0] as usize);
    });

    let mut ans: Vec<i32> = vec![];
    loop {
      let mut idx: usize = 0;
      while idx < nc {
        if cnt[idx] == 0 {
          cnt[idx] = -1;
          break;
        }
        idx += 1;
      }
      if idx == nc {
        break;
      }
      ans.push(idx as i32);
      for n in &deps[idx] {
        cnt[*n] -= 1;
      }
    }
    if ans.len() == nc {
      ans
    } else {
      vec![]
    }
  }
}
