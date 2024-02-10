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
  pub fn inorder_traversal2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();

    fn recur(root: Option<Rc<RefCell<TreeNode>>>, ret: &mut Vec<i32>) {
      if root.is_none() {
        return;
      }

      recur(root.as_ref().unwrap().borrow().left.clone(), ret);
      ret.push(root.as_ref().unwrap().borrow().val);
      recur(root.as_ref().unwrap().borrow().right.clone(), ret);
    }

    recur(root, &mut ret);
    ret
  }
}
