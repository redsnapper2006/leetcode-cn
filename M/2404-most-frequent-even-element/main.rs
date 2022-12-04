struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn most_frequent_even(nums: Vec<i32>) -> i32 {
    let mut hm: HashMap<i32, i32> = HashMap::new();

    nums.into_iter().filter(|x| x % 2 == 0).for_each(|n| {
      hm.entry(n).and_modify(|c| *c += 1).or_insert(1);
    });

    let ret = hm.iter().fold((-1, 0), |(key, cnt), (k, v)| {
      if cnt < *v || cnt == *v && *k < key {
        return (*k, *v);
      }
      (key, cnt)
    });

    ret.0
  }
}

fn main() {
  println!("{}", Solution::most_frequent_even(vec![1, 2, 3, 4, 5, 6]));
}
