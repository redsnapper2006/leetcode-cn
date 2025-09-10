use std::collections::HashSet;
impl Solution {
  pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
    let mut cnt: Vec<i32> = vec![0; n as usize + 1];
    let mut visit: HashSet<usize> = HashSet::new();
    let mut total: i32 = 0;
    for fs in friendships.iter() {
      let fs1 = fs[0] as usize - 1;
      let fs2 = fs[1] as usize - 1;

      let set1: HashSet<_> = languages[fs1].clone().into_iter().collect();
      let set2: HashSet<_> = languages[fs2].clone().into_iter().collect();

      let intersection: Vec<_> = set1.intersection(&set2).cloned().collect();
      if intersection.len() > 0 {
        continue;
      }
      if !visit.contains(&fs1) {
        total += 1;
        visit.insert(fs1);
        for n in languages[fs1].iter() {
          cnt[*n as usize] += 1;
        }
      }
      if !visit.contains(&fs2) {
        total += 1;
        visit.insert(fs2);
        for n in languages[fs2].iter() {
          cnt[*n as usize] += 1;
        }
      }
    }

    total - cnt.iter().max().unwrap()
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::minimum_teachings(3, vec![], vec![]));
}
