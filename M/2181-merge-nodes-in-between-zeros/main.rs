struct Solution {}

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

impl Solution {
  pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut nh: Option<Box<ListNode>> = None;
    let mut nhp = &mut nh;
    let mut p = head;
    let mut sum: i32 = 0;
    while p.is_some() {
      let pp = p.unwrap();
      if pp.val == 0 && sum != 0 {
        *nhp = Some(Box::new(ListNode::new(sum)));
        nhp = &mut nhp.as_mut().unwrap().next;
        sum = 0;
      } else {
        sum += pp.val;
      }
      p = pp.next;
    }
    nh
  }
}
