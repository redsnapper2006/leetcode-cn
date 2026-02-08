use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn recur(node: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
      if None == node {
        return (true, 0);
      }

      let left = recur(node.as_ref().unwrap().borrow().left.clone());
      let right = recur(node.as_ref().unwrap().borrow().right.clone());
      (left.0 && right.0 && (left.1 - right.1).abs() <= 1, left.1.max(right.1) + 1)
    }
    recur(root).0
  }
}

// Definition for a binary tree node.
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
