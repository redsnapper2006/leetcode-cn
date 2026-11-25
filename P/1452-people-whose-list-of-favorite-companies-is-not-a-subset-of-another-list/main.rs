use std::collections::HashSet;

impl Solution {
  pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
    let mut fav_sets: Vec<(usize, HashSet<String>)> = favorite_companies
      .into_iter()
      .enumerate()
      .map(|(i, fav)| (i, fav.into_iter().collect()))
      .collect();
    fav_sets.sort_by(|b, a| b.1.len().cmp(&a.1.len()));

    let mut ans: Vec<i32> = vec![];
    for (pos_i, (org_i, s)) in fav_sets.iter().enumerate() {
      let is_sub = fav_sets.iter().skip(pos_i + 1).any(|(_, t)| s.is_subset(t));
      if !is_sub {
        ans.push(*org_i as i32);
      }
    }
    ans.sort_unstable();
    ans
  }
}
