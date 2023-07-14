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
  pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn recur(root: Option<Rc<RefCell<TreeNode>>>, cnt: &mut i32) -> i32 {
      if let Some(node) = root {
        let left = recur(node.borrow_mut().left.take(), cnt);
        let right = recur(node.borrow_mut().right.take(), cnt);
        let v = left + right + node.borrow().val - 1;
        *cnt += v.abs();
        v
      } else {
        0
      }
    }
    let mut cnt: i32 = 0;
    recur(root, &mut cnt);
    cnt
  }
}
