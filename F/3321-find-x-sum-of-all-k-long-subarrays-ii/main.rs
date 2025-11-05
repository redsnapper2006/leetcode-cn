use std::collections::{BTreeMap, BTreeSet};

impl Solution {
  pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    let k = k as usize;
    let x = x as usize;

    let mut queue: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut backup: BTreeSet<(i64, i64)> = BTreeSet::new();
    let mut map: BTreeMap<i64, i64> = BTreeMap::new();
    (0..k).for_each(|idx| {
      map
        .entry(nums[idx] as i64)
        .and_modify(|x| *x += 1)
        .or_insert(1);
    });
    let mut q: BTreeSet<(i64, i64)> = BTreeSet::new();
    for (k, v) in map.iter() {
      q.insert((*v, *k));
    }
    let mut it = q.iter();
    (0..(map.len() - x.min(map.len()))).for_each(|_| {
      let (v, k) = it.next().unwrap();
      backup.insert((*v, *k));
    });

    let mut sum: i64 = 0;
    while let Some((v, k)) = it.next() {
      queue.insert((*v, *k));
      sum += (*v) * (*k);
    }

    let mut ans: Vec<i64> = vec![sum];
    (k..nums.len()).for_each(|idx| {
      let prev = nums[idx - k] as i64;
      let cnt = map.get_mut(&prev).unwrap();
      *cnt -= 1;
      if queue.contains(&(*cnt + 1, prev)) {
        queue.remove(&(*cnt + 1, prev));
        queue.insert((*cnt, prev));
        sum -= prev;
      } else {
        backup.remove(&(*cnt + 1, prev));
        backup.insert((*cnt, prev));
      }

      let cur = nums[idx] as i64;
      map.entry(cur).and_modify(|x| *x += 1).or_insert(1);
      let cnt2 = *map.get(&cur).unwrap();
      if queue.len() < x || queue.contains(&(cnt2 - 1, cur)) {
        queue.remove(&(cnt2 - 1, cur));
        queue.insert((cnt2, cur));
        sum += cur;
      } else {
        backup.remove(&(cnt2 - 1, cur));
        backup.insert((cnt2, cur));
      }

      if queue.len() == x && backup.len() > 0 {
        let qfirst = *queue.first().unwrap();
        let blast = *backup.last().unwrap();
        if blast > qfirst {
          queue.remove(&qfirst);
          backup.remove(&blast);
          queue.insert(blast);
          backup.insert(qfirst);
          sum += blast.0 * blast.1;
          sum -= qfirst.0 * qfirst.1;
        }
      }

      ans.push(sum);
    });

    ans
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::find_x_sum(vec![1, 1, 2, 2, 3, 4, 2, 3], 6, 2)
  );
  println!("{:?}", Solution::find_x_sum(vec![3, 8, 7, 8, 7, 5], 2, 2));
  println!("{:?}", Solution::find_x_sum(vec![2, 2, 3, 3, 4, 2], 3, 3));
}

struct Solution {}
