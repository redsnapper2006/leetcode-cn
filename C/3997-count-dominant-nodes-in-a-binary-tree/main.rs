use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn count_dominant_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn recur(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
      if let Some(n) = node {
        let n = n.borrow();
        let left = n.left.clone();
        let right = n.right.clone();
        let mx = n.val.max(recur(left, ans)).max(recur(right, ans));
        *ans += if mx == n.val { 1 } else { 0 };
        mx
      } else {
        0
      }
    }
    let mut ans: i32 = 0;
    recur(root, &mut ans);
    ans
  }
}
