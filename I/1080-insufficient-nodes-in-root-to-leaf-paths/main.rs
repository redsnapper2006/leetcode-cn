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
  pub fn sufficient_subset(
    root: Option<Rc<RefCell<TreeNode>>>,
    limit: i32,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    let mut head: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(0))));
    head.as_ref().unwrap().borrow_mut().left = root;

    fn recur(root: Option<Rc<RefCell<TreeNode>>>, limit: i32, sum: i32) -> bool {
      let v = sum + root.as_ref().unwrap().borrow().val;
      if root.as_ref().unwrap().borrow().left == None
        && root.as_ref().unwrap().borrow().right == None
      {
        return v >= limit;
      }
      let mut left: bool = false;
      let mut right: bool = false;
      if let Some(ls) = root.as_ref().unwrap().borrow().left.clone() {
        left = recur(Some(ls), limit, v);
      }
      if !left {
        root.as_ref().unwrap().borrow_mut().left = None;
      }

      if let Some(ls) = root.as_ref().unwrap().borrow().right.clone() {
        right = recur(Some(ls), limit, v);
      }
      if !right {
        root.as_ref().unwrap().borrow_mut().right = None;
      }
      left || right
    }

    fn recur2(root: Option<Rc<RefCell<TreeNode>>>, limit: i32, sum: i32) -> bool {
      match root {
        None => false,
        Some(node) => {
          let v = sum + node.borrow().val;
          if node.borrow().left == None && node.borrow().right == None {
            return v >= limit;
          }
          let mut left: bool = false;
          let mut right: bool = false;
          if let Some(ls) = node.borrow().left.clone() {
            left = recur(Some(ls), limit, v);
          }
          if !left {
            node.borrow_mut().left = None;
          }

          if let Some(ls) = node.borrow().right.clone() {
            right = recur(Some(ls), limit, v);
          }
          if !right {
            node.borrow_mut().right = None;
          }
          left || right
        }
      }
    }
    recur(head.clone(), limit, 0);
    head.unwrap().borrow_mut().left.clone()
  }
}
