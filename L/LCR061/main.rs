use std::cmp::Reverse;
use std::collections::BinaryHeap;
impl Solution {
  pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

    for i in 0..nums1.len() {
      heap.push(Reverse((nums1[i] + nums2[0], i, 0)));
    }

    let mut ans: Vec<Vec<i32>> = vec![];
    let mut k = k;
    while k > 0 && heap.len() > 0 {
      let (_, i1, i2) = heap.pop().unwrap().0;
      ans.push(vec![nums1[i1], nums2[i2]]);
      if i2 < nums2.len() - 1 {
        heap.push(Reverse((nums1[i1] + nums2[i2 + 1], i1, i2 + 1)));
      }
      k -= 1;
    }

    ans
  }
}
