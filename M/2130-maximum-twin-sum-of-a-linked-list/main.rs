impl Solution {
  pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut buf: Vec<i32> = vec![];
    let mut h = head;
    while let Some(ln) = h {
      buf.push(ln.val);
      h = ln.next;
    }

    (0..buf.len() / 2).fold(i32::MIN, |ans, idx| ans.max(buf[idx] + buf[buf.len() - 1 - idx]))
  }
}

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
