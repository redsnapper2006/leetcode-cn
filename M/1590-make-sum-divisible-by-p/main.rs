use std::collections::HashMap;

impl Solution {
  pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let mut buf: Vec<i32> = vec![];
    let mut sum: i32 = 0;
    nums.iter().for_each(|x| {
      sum += x;
      sum %= p;
      buf.push(sum);
    });
    if sum == 0 {
      return 0;
    }

    let mut m: HashMap<i32, i32> = HashMap::new();
    m.insert(0, -1);
    let mut ret = nums.len() as i32;
    (0..nums.len()).for_each(|idx| {
      let offset = (buf[idx] - sum + p) % p;
      if m.contains_key(&offset) {
        let prev = m.get(&offset).unwrap();
        if ret > idx as i32 - prev {
          ret = idx as i32 - prev;
        }
      }
      m.insert(buf[idx], idx as i32);
    });
    if ret == nums.len() as i32 {
      -1
    } else {
      ret as i32
    }
  }
}
