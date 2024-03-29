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
  pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    root.as_ref().unwrap().borrow().val
      == root
        .as_ref()
        .unwrap()
        .borrow()
        .left
        .as_ref()
        .unwrap()
        .borrow()
        .val
        + root
          .as_ref()
          .unwrap()
          .borrow()
          .right
          .as_ref()
          .unwrap()
          .borrow()
          .val
  }
}
