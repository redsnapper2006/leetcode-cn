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
  pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut idx: Vec<usize> = Vec::new();
    let mut p = head;

    let mut ret: Vec<i32> = Vec::new();
    let mut offset: usize = 0;
    while let Some(sp) = p {
      while stack.len() > 0 && sp.val > stack[stack.len() - 1] {
        stack.pop();
        ret[idx.pop().unwrap()] = sp.val;
      }

      stack.push(sp.val);
      idx.push(offset);
      ret.push(0);
      offset += 1;
      p = sp.next;
    }

    ret
  }
}
