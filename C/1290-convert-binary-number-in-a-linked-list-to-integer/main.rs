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
  pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut h = head;
    let mut ans: i32 = 0;

    while h.is_some() {
      ans = ans * 2 + h.as_mut().unwrap().val;
      h = h.as_mut().unwrap().next.clone();
    }
    ans
  }
}
