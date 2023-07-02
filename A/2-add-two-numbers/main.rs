struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

impl Solution {
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut h = &mut head;
    let mut is_over: bool = false;
    let mut p1 = l1;
    let mut p2 = l2;
    while p1.is_some() || p2.is_some() || is_over {
      let mut t1: i32 = 0;
      let mut t2: i32 = 0;
      if p1.is_some() {
        let pp = p1.unwrap();
        t1 = pp.val;
        p1 = pp.next;
      }
      if p2.is_some() {
        let pp = p2.unwrap();
        t2 = pp.val;
        p2 = pp.next;
      }
      let mut t = t1 + t2;
      if is_over {
        t += 1;
      }
      if t >= 10 {
        is_over = true;
        t -= 10;
      } else {
        is_over = false;
      }

      h.as_mut().unwrap().val = t;
      if p1.is_some() || p2.is_some() || is_over {
        h.as_mut().unwrap().next = Some(Box::new(ListNode::new(0)));
      } else {
        h.as_mut().unwrap().next = None;
      }
      h = &mut h.as_mut().unwrap().next;
    }
    head
  }
}
