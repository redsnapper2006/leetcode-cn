struct Solution {}

use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
  pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    let mut ban: HashSet<i32> = HashSet::new();
    banned.iter().for_each(|&v| {
      ban.insert(v);
    });
    let mut sets = vec![BTreeSet::new(), BTreeSet::new()];
    for i in 0..n {
      if i != p && !ban.contains(&(i)) {
        sets[i as usize % 2].insert(i);
      }
    }

    let mut ans: Vec<i32> = vec![-1; n as usize];
    ans[p as usize] = 0;
    q.push_back((p, 0));
    while q.len() > 0 {
      let (idx, step) = q.pop_front().unwrap();
      let mn = (idx - k + 1).max(k - idx - 1);
      let mx = (idx + k - 1).min(n * 2 - k - idx - 1);
      let set = &mut sets[mx as usize % 2];
      let mut removed: Vec<i32> = vec![];
      set.range(mn..=mx).for_each(|&off| {
        ans[off as usize] = step + 1;
        q.push_back((off, step + 1));
        removed.push(off);
      });
      removed.iter().for_each(|&v| {
        set.remove(&v);
      });
    }

    ans
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::min_reverse_operations(4, 0, vec![1, 2], 4)
  );
  println!(
    "{:?}",
    Solution::min_reverse_operations(5, 0, vec![2, 4], 3)
  );
  println!(
    "{:?}",
    Solution::min_reverse_operations(4, 2, vec![0, 1, 3], 1)
  );
}
