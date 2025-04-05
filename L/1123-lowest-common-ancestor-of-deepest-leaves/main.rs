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
  pub fn lca_deepest_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn recur(
      root: Option<Rc<RefCell<TreeNode>>>,
      depth: i32,
    ) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
      let rr = root.unwrap();
      let r = rr.borrow();
      if r.left.is_none() && r.right.is_none() {
        return (Some(rr.clone()), depth);
      }
      let mut ld: i32 = 0;
      let mut rd: i32 = 0;
      let mut lp: Option<Rc<RefCell<TreeNode>>> = None;
      let mut rp: Option<Rc<RefCell<TreeNode>>> = None;

      if r.left.is_some() {
        (lp, ld) = recur(r.left.clone(), depth + 1);
      }
      if r.right.is_some() {
        (rp, rd) = recur(r.right.clone(), depth + 1);
      }
      if ld == rd {
        return (Some(rr.clone()), ld);
      }
      if ld > rd {
        return (lp, ld);
      }
      (rp, rd)
    }

    let (ans, _) = recur(root, 0);

    ans
  }
}
