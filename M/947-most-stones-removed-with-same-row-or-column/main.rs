
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
  pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
    let mut x: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut y: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    stones.iter().for_each(|stone| {
      x.entry(stone[0]).or_insert(vec![]).push(stone[1]);
      y.entry(stone[1]).or_insert(vec![]).push(stone[0]);
    });

    let mut colors: HashMap<i32, i32> = HashMap::new();
    let mut color: i32 = 0;
    x.iter().for_each(|(r, v)| {
      v.iter().for_each(|c| {
        if visited.contains(&(*r, *c)) {
          return;
        }
        color += 1;
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((*r, *c));
        visited.insert((*r, *c));
        *colors.entry(color).or_insert(0) += 1;

        while !q.is_empty() {
          let (r, c) = q.pop_front().unwrap();

          if let Some(v) = x.get(&r) {
            v.iter().for_each(|&c| {
              if !visited.contains(&(r, c)) {
                visited.insert((r, c));
                *colors.entry(color).or_insert(0) += 1;
                q.push_back((r, c));
              }
            });
          }

          if let Some(v) = y.get(&c) {
            v.iter().for_each(|&r| {
              if !visited.contains(&(r, c)) {
                visited.insert((r, c));
                *colors.entry(color).or_insert(0) += 1;
                q.push_back((r, c));
              }
            });
          }
        }
      });
    });

    let mut cnt: i32 = 0;
    colors.iter().for_each(|(_, v)| {
      cnt += v - 1;
    });

    cnt
  }
}
