use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
  pub fn watched_videos_by_friends(
    watched_videos: Vec<Vec<String>>,
    friends: Vec<Vec<i32>>,
    id: i32,
    level: i32,
  ) -> Vec<String> {
    let mut visit: HashSet<i32> = HashSet::new();
    let mut vc: HashMap<String, i32> = HashMap::new();

    let mut q: VecDeque<(i32, i32)> = VecDeque::new();
    q.push_back((id, 0));
    visit.insert(id);
    while q.len() > 0 {
      let (id, le) = q.pop_front().unwrap();
      if le == level {
        for v in &watched_videos[id as usize] {
          vc.entry(v.clone()).and_modify(|x| *x += 1).or_insert(1);
        }
        continue;
      }

      for n in &friends[id as usize] {
        if visit.contains(&n) {
          continue;
        }
        visit.insert(*n);
        q.push_back((*n, le + 1));
      }
    }

    let mut t: Vec<(i32, String)> = vec![];
    for (k, v) in vc {
      t.push((v, k));
    }
    t.sort_unstable();
    let mut ans: Vec<String> = vec![];
    for (_, k) in t {
      ans.push(k);
    }
    ans
  }
}
