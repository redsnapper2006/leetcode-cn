struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn dfs(buf: &mut Vec<i32>, m: &HashMap<usize, Vec<usize>>, index: usize, parent: usize) {
    if !m.contains_key(&index) {
      return;
    }
    let children = m.get(&index).unwrap();
    for i in 0..children.len() {
      if buf[children[i]] != -1 {
        continue;
      }
      buf[children[i]] = parent as i32;
      Solution::dfs(buf, m, children[i], parent);
    }
  }

  pub fn num_similar_groups(strs: Vec<String>) -> i32 {
    let mut m: HashMap<usize, Vec<usize>> = HashMap::new();
    for i in 0..strs.len() {
      let target = strs[i].as_bytes();
      for j in i + 1..strs.len() {
        let candi = strs[j].as_bytes();
        let mut is_first = true;
        let mut is_second = true;
        let mut is_target = false;
        let mut diff1: u8 = '0' as u8;
        let mut diff2: u8 = '0' as u8;

        for n in 0..target.len() {
          if target[n] != candi[n] {
            if is_first {
              is_first = false;
              diff1 = target[n];
              diff2 = candi[n];
            } else if is_second && diff1 == candi[n] && diff2 == target[n] {
              is_second = false;
              is_target = true;
            } else {
              is_target = false;
              break;
            }
          }
        }
        if is_target || is_first {
          if !m.contains_key(&i) {
            m.insert(i, vec![j]);
          } else {
            m.get_mut(&i).unwrap().push(j);
          }
          if !m.contains_key(&j) {
            m.insert(j, vec![i]);
          } else {
            m.get_mut(&j).unwrap().push(i);
          }
        }
      }
    }

    let mut buf: Vec<i32> = vec![-1; strs.len()];
    for i in 0..strs.len() {
      if buf[i] != -1 {
        continue;
      }
      buf[i] = i as i32;
      Solution::dfs(&mut buf, &m, i, i);
    }

    let mut r: HashMap<usize, usize> = HashMap::new();
    for i in 0..buf.len() {
      if !r.contains_key(&(buf[i] as usize)) {
        r.insert(buf[i] as usize, i);
      }
    }
    r.len() as i32
  }
}

fn main() {
  println!(
    "{}",
    Solution::num_similar_groups(vec![
      String::from("tars"),
      String::from("rats"),
      String::from("arts"),
      String::from("star")
    ])
  );
}
