use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn recur(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
      match node {
        None => (0, 0, 0),
        Some(n) => {
          let val = n.borrow().val;
          let (left_sum, left_cnt, left_ans) = recur(n.borrow_mut().left.take());
          let (right_sum, right_cnt, right_ans) = recur(n.borrow_mut().right.take());
          let sum = left_sum + right_sum + val;
          let cnt = left_cnt + right_cnt + 1;
          let ans = left_ans + right_ans + if sum / cnt == val { 1 } else { 0 };
          (sum, cnt, ans)
        }
      }
    }
    recur(root).2
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
