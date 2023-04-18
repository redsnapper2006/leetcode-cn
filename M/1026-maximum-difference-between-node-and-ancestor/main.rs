struct Solution {}

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
use std::rc::Rc;
impl Solution {
  pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res: i32 = 0;

    fn recur(root: Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32, res: &mut i32) {
      match root {
        None => (),
        Some(rc) => {
          let mut min_v = min;
          let mut max_v = max;
          let v = rc.borrow().val;
          if min_v > v {
            min_v = v;
          }
          if max_v < v {
            max_v = v;
          }
          if *res < max_v - min_v {
            *res = max_v - min_v;
          }
          recur(rc.borrow().left.clone(), min_v, max_v, res);
          recur(rc.borrow().right.clone(), min_v, max_v, res);
        }
      }
    }
    recur(root, 100000, 0, &mut res);
    res
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_ancestor_diff(Some(Rc::new(RefCell::new(TreeNode::new(1)))))
  );
}
