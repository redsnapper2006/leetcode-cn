use std::collections::HashMap;
struct Solution {
  ns: Vec<i32>,
  hm: HashMap<i32, usize>,
}

impl Solution {
  fn new(nums: Vec<i32>) -> Self {
    Solution {
      ns: nums,
      hm: HashMap::new(),
    }
  }

  fn pick(&mut self, target: i32) -> i32 {
    let mut idx: usize = 0;
    if self.hm.contains_key(&target) {
      idx = *(self.hm.get(&target).unwrap());
    }
    loop {
      idx += 1;
      idx %= self.ns.len();

      if self.ns[idx] == target {
        break;
      }
    }
    self.hm.insert(target, idx);
    idx as i32
  }
}

fn main() {
  let obj = Solution::new(nums);
  let ret_1: i32 = obj.pick(target);
  println!("{}", ret_1);
}
