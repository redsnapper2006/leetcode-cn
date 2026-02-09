use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
  pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn collect(node: Option<Rc<RefCell<TreeNode>>>, coll: &mut Vec<i32>) {
      if None == node {
        return;
      }
      let node = node.unwrap();
      collect(node.borrow().left.clone(), coll);
      coll.push(node.borrow().val);
      collect(node.borrow().right.clone(), coll);
    }

    fn setup(coll: &Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
      if coll.len() == 0 {
        return None;
      }

      let m = coll.len() / 2;
      Some(Rc::new(RefCell::new(TreeNode {
        val: coll[m],
        left: setup(&coll[0..m].to_vec()),
        right: setup(&coll[m + 1..coll.len()].to_vec()),
      })))
    }
    let mut coll: Vec<i32> = vec![];
    collect(root, &mut coll);
    setup(&coll)
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
