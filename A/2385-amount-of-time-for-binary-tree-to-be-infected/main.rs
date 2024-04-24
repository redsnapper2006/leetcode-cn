#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None,
    }
  }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
  pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, m: &mut HashMap<i32, Vec<i32>>) {
      let binding = root.unwrap();
      let rr = binding.borrow();
      let v = rr.val;
      if !m.contains_key(&v) {
        m.insert(v, vec![]);
      }

      if rr.left.is_some() {
        let l = rr.left.as_ref().unwrap().borrow().val;
        m.get_mut(&v).unwrap().push(l);
        if !m.contains_key(&l) {
          m.insert(l, vec![]);
        }
        m.get_mut(&l).unwrap().push(v);
        dfs(rr.left.clone(), m);
      }
      if rr.right.is_some() {
        let r = rr.right.as_ref().unwrap().borrow().val;
        m.get_mut(&v).unwrap().push(r);
        if !m.contains_key(&r) {
          m.insert(r, vec![]);
        }
        m.get_mut(&r).unwrap().push(v);
        dfs(rr.right.clone(), m);
      }
    }

    dfs(root, &mut m);

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut visited: HashSet<i32> = HashSet::new();
    q.push_back((start, 0));
    visited.insert(start);

    let mut max: i32 = 0;
    while q.len() > 0 {
      let next = q.pop_front().unwrap();
      let v = next.0;
      let d = next.1;
      if d > max {
        max = d;
      }
      for n in m.get(&v).unwrap() {
        if !visited.contains(n) {
          q.push_back((*n, d + 1));
          visited.insert(*n);
        }
      }
    }
    max
  }
}
