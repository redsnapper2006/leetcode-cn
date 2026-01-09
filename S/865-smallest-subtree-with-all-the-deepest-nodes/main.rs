// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn subtree_with_all_deepest(
    root: Option<Rc<RefCell<TreeNode>>>,
  ) -> Option<Rc<RefCell<TreeNode>>> {
    fn recur(root: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
      if root.is_none() {
        return (root, 0);
      }
      let binding = root.clone().unwrap();
      let b = binding.borrow();

      let (l, ld) = recur(b.left.clone());
      let (r, rd) = recur(b.right.clone());
      if ld < rd {
        (r, rd + 1)
      } else if ld > rd {
        (l, ld + 1)
      } else {
        (root.clone(), ld + 1)
      }
    }
    recur(root).0
  }
}
