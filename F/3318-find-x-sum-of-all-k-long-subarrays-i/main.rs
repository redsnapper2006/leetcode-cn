use std::collections::BTreeMap;
use std::collections::HashMap;

impl Solution {
  pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let x = x as usize;
    let mut tm: BTreeMap<String, (i32, i32)> = BTreeMap::new();
    let mut hm: HashMap<i32, i32> = HashMap::new();

    (0..k).for_each(|idx| {
      hm.entry(nums[idx]).and_modify(|x| *x += 1).or_insert(1);
    });
    for (k, v) in &hm {
      tm.insert(format!("{:010}_{:010}", *v, *k), (*v, *k));
    }

    let mut sum: i32 = 0;
    let keys = tm.keys().cloned().collect::<Vec<String>>();
    (0..x.min(keys.len())).for_each(|idx| {
      let v = &keys[keys.len() - 1 - idx];
      let k = tm.get(v).unwrap();
      sum += k.0 * k.1;
    });

    let mut ans: Vec<i32> = vec![sum];
    (k..nums.len()).for_each(|idx| {
      let cur = nums[idx];
      if hm.contains_key(&cur) {
        tm.remove(&format!("{:010}_{:010}", *hm.get(&cur).unwrap(), cur));
      }
      hm.entry(cur).and_modify(|x| *x += 1).or_insert(1);
      tm.insert(
        format!("{:010}_{:010}", *hm.get(&cur).unwrap(), cur),
        (*hm.get(&cur).unwrap(), cur),
      );
      let prev = nums[idx - k];
      if hm.contains_key(&prev) {
        tm.remove(&format!("{:010}_{:010}", *hm.get(&prev).unwrap(), prev));
      }
      hm.entry(prev).and_modify(|x| *x -= 1);
      tm.insert(
        format!("{:010}_{:010}", *hm.get(&prev).unwrap(), prev),
        (*hm.get(&prev).unwrap(), prev),
      );

      let mut sum: i32 = 0;
      let mut keys = tm.keys().cloned().collect::<Vec<String>>();
      (0..x.min(keys.len())).for_each(|idx| {
        let mut v = &keys[keys.len() - 1 - idx];
        let mut k = *tm.get(v).unwrap();
        sum += k.0 * k.1;
      });
      ans.push(sum);
    });
    ans
  }
}
