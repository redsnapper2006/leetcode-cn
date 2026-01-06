use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
  pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut buf: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
    let mut mx: i32 = i32::MIN;
    let mut level: i32 = 0;
    let mut ans: i32 = -1;

    while buf.len() > 0 {
      level += 1;
      let mut t: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
      let mut sum: i32 = 0;
      for bb in buf {
        let binding = bb.unwrap();
        let b = binding.borrow();
        sum += b.val;
        if b.left.is_some() {
          t.push(b.left.clone());
        }
        if b.right.is_some() {
          t.push(b.right.clone());
        }
      }
      if mx < sum {
        mx = sum;
        ans = level;
      }
      buf = t;
    }
    ans
  }
}

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
