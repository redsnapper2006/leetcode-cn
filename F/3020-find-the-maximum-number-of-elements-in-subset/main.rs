use std::collections::HashMap;

impl Solution {
  pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, (i32, i32)> = HashMap::new();
    nums.iter().for_each(|&v| {
      m.entry(v).or_insert((0, 1)).0 += 1;
    });

    let mut ans: i32 = 1;
    if let Some(&(cnt, _)) = m.get(&1) {
      ans = ans.max((cnt + 1) / 2);
    }

    let mut keys: Vec<i32> = m.keys().copied().collect();
    keys.sort_unstable();
    for &v in &keys {
      if v == 1 || m[&v].0 < 2 {
        continue;
      }

      let n = v * v;
      let l = m[&v].1;
      if let Some(entry) = m.get_mut(&n) {
        entry.1 = l + 1;
        ans = ans.max(entry.1);
      }
    }
    ans * 2 - 1
  }
}
