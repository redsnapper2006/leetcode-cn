impl Solution {
  pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut pos = 2i32;

    let p = head.unwrap();
    let mut prev = p.val;
    let mut p = p.next.unwrap();
    let mut min_pos = 0i32;
    let mut prev_pos = 0i32;
    let mut min_dist = i32::MAX;

    while let Some(n) = p.next {
      if p.val > prev && p.val > n.val || p.val < prev && p.val < n.val {
        if min_pos == -1 {
          min_pos = pos;
        }
        if prev_pos != 0 {
          min_dist = min_dist.min(pos - prev_pos);
        }
        prev_pos = pos;
      }

      prev = p.val;
      p = n;
      pos += 1;
    }

    if min_dist == i32::MAX {
      vec![-1, -1]
    } else {
      vec![min_dist, prev_pos - min_pos]
    }
  }
}

// Definition for singly-linked list.
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
