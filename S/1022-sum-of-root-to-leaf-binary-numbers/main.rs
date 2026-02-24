use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn recur(root: &Option<Rc<RefCell<TreeNode>>>, aggr: i32, ans: &mut i32) {
      match root {
        Some(root) => {
          let aggr = aggr * 2 + root.borrow().val;

          if let None = root.borrow().left
            && let None = root.borrow().right
          {
            *ans += aggr;
          } else {
            recur(&root.borrow().left, aggr, ans);
            recur(&root.borrow().right, aggr, ans);
          }
        }
        _ => (),
      }
    }

    let mut ans: i32 = 0;
    recur(&root, 0, &mut ans);
    ans
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
