use std::collections::HashMap;
impl Solution {
  pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    for i in 0..nums.len() {
      m.entry(nums[i])
        .and_modify(|x| x.push(i as i32))
        .or_insert(vec![i as i32]);
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
