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
    let mut ll1 = l1;
    let mut ll2 = l2;
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    while let Some(node) = ll1 {
      v1.push(node.val);
      ll1 = node.next;
    }
    while let Some(node) = ll2 {
      v2.push(node.val);
      ll2 = node.next;
    }

    let mut head: Option<Box<ListNode>> = None;
    let mut is_over: bool = false;
    while !v1.is_empty() && !v2.is_empty() {
      let mut v: i32 = v1.pop().unwrap() + v2.pop().unwrap();
      if is_over {
        v += 1;
      }
      if v > 9 {
        is_over = true;
        v -= 10;
      } else {
        is_over = false;
      }

      let mut n = Box::new(ListNode::new(v));
      n.next = head;
      head = Some(n);
    }

    while let Some(mut v) = v1.pop() {
      if is_over {
        v += 1;
      }
      if v > 9 {
        is_over = true;
        v -= 10;
      } else {
        is_over = false;
      }
      let mut n = Box::new(ListNode::new(v));
      n.next = head;
      head = Some(n);
    }

    while let Some(mut v) = v2.pop() {
      if is_over {
        v += 1;
      }
      if v > 9 {
        is_over = true;
        v -= 10;
      } else {
        is_over = false;
      }
      let mut n = Box::new(ListNode::new(v));
      n.next = head;
      head = Some(n);
    }
    if is_over {
      let mut n = Box::new(ListNode::new(1));
      n.next = head;
      head = Some(n);
    }

    head
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::add_two_numbers(Some(Box::new(ListNode::new(3))), None)
  );
}
