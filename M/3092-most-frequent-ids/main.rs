use std::collections::{BinaryHeap, HashMap};

impl Solution {
  pub fn most_frequent_i_ds(nums: Vec<i32>, freq: Vec<i32>) -> Vec<i64> {
    let mut cnt: HashMap<i64, i64> = HashMap::new();
    let mut heap: BinaryHeap<(i64, i64)> = BinaryHeap::new();

    let mut ans: Vec<i64> = vec![];
    for i in 0..nums.len() {
      *cnt.entry(nums[i] as i64).or_insert(0) += freq[i] as i64;
      heap.push((*cnt.get(&(nums[i] as i64)).unwrap(), nums[i] as i64));

      while let Some((c, n)) = heap.peek()
        && c != cnt.get(&n).unwrap()
      {
        heap.pop();
      }
      ans.push(heap.peek().unwrap().0);
    }
    ans
  }
}
