impl Solution {
  pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut h = &head;
    let mut cnt: i32 = 0;
    while let Some(ln) = h {
      cnt += 1;
      h = &ln.next;
    }

    cnt /= 2;
    let mut nh = Box::new(ListNode { val: 0, next: head });
    let mut pp = &mut nh;
    while cnt > 0 {
      pp = pp.next.as_mut()?;
      cnt -= 1;
    }
    pp.next = pp.next.as_mut()?.next.take();

    nh.next
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
