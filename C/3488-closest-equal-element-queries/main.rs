use std::collections::HashMap;
impl Solution {
  pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut m: HashMap<i32, i32> = HashMap::new();

    let mut left: Vec<i32> = vec![-1; nums.len()];
    let mut right: Vec<i32> = vec![-1; nums.len()];

    (0..nums.len() * 2).for_each(|idx| {
      if idx >= n {
        left[idx % n] = if m.contains_key(&nums[idx % n]) {
          *m.get(&nums[idx % n]).unwrap()
        } else {
          0
        };
      }
      m.insert(nums[idx % n], idx as i32 - n as i32);
    });

    m.clear();
    (0..nums.len() * 2).rev().for_each(|idx| {
      if idx < n {
        right[idx] = if m.contains_key(&nums[idx]) {
          *m.get(&nums[idx]).unwrap()
        } else {
          0
        };
      }
      m.insert(nums[idx % n], idx as i32);
    });

    queries
      .iter()
      .map(|&q| {
        if q - left[q as usize] == nums.len() as i32 {
          -1
        } else {
          (q - left[q as usize]).min(right[q as usize] - q)
        }
      })
      .collect()
  }

  pub fn solve_queries2(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..nums.len() {
      m.entry(nums[i]).and_modify(|x| x.push(i as i32)).or_insert(vec![i as i32]);
    }

    let mut ans: Vec<i32> = vec![];
    queries.iter().for_each(|&q| {
      let off = m.get(&nums[q as usize]).unwrap();
      if off.len() == 1 {
        ans.push(-1);
        return;
      }
      let mut pos: i32 = nums.len() as i32 + 1;
      let rr = match off.binary_search(&q) {
        Ok(ov) => ov,
        _ => nums.len() + 1,
      };

      if rr == 0 {
        ans.push((off[1] - off[0]).min(nums.len() as i32 - off[off.len() - 1] + off[0]));
      } else if rr == off.len() - 1 {
        ans.push((off[rr] - off[rr - 1]).min(nums.len() as i32 - off[rr] + off[0]));
      } else {
        ans.push((off[rr] - off[rr - 1]).min(off[rr + 1] - off[rr]));
      }
    });
    ans
  }
}
