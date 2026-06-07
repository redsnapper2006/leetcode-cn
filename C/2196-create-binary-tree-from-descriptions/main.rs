use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
  pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut m: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
    let mut e: HashMap<i32, bool> = HashMap::new();
    descriptions.iter().for_each(|desc| {
      let r = m.entry(desc[0]).or_insert(Rc::new(RefCell::new(TreeNode::new(desc[0])))).clone();
      let c = m.entry(desc[1]).or_insert(Rc::new(RefCell::new(TreeNode::new(desc[1])))).clone();
      e.insert(desc[1], true);
      if desc[2] == 1 {
        r.borrow_mut().left = Some(c);
      } else {
        r.borrow_mut().right = Some(c);
      }
    });

    for desc in &descriptions {
      if !e.contains_key(&desc[0]) {
        return Some(Rc::clone(m.get(&desc[0]).unwrap()));
      }
    }
    None
  }
}

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
