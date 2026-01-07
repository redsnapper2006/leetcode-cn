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
  pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn sum(node: Option<Rc<RefCell<TreeNode>>>) -> i64 {
      if node.is_none() {
        0
      } else {
        let binding = node.unwrap();
        let b = binding.borrow();
        b.val as i64 + sum(b.left.clone()) + sum(b.right.clone())
      }
    }

    let total = sum(root.clone());

    fn calc(node: Option<Rc<RefCell<TreeNode>>>, total: i64, ans: &mut i64) -> i64 {
      if node.is_none() {
        return 0;
      }
      let binding = node.unwrap();
      let b = binding.borrow();
      let v = b.val as i64 + calc(b.left.clone(), total, ans) + calc(b.right.clone(), total, ans);
      *ans = (*ans).max((total - v) * v);
      v
    }
    let mut ans: i64 = 0;
    calc(root, total, &mut ans);
    (ans % 1000000007) as _
  }
}
