struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, i32> = HashMap::new();
    let mut p: HashMap<i32, usize> = HashMap::new();
    let mut ret = nums.len() + 1;
    let mut maxcnt = 0;
    for i in 0..nums.len() {
      let cnt = m.entry(nums[i]).or_insert(0);
      *cnt += 1;
      if *cnt > maxcnt {
        maxcnt = *cnt
      }
      p.entry(nums[i]).or_insert(i);
    }
    for i in (0..nums.len()).rev() {
      if m.contains_key(&nums[i]) && *(m.get(&nums[i]).unwrap()) == maxcnt {
        m.remove(&nums[i]);
        if i - *(p.get(&nums[i]).unwrap()) < ret {
          ret = i - p.get(&nums[i]).unwrap();
        }
      }
    }
    ret as i32 + 1
  }
}

fn main() {
  println!("{}",Solution::find_shortest_sub_array(vec![0;5]);)
}
