struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut cache: HashMap<i32, HashMap<i32, i32>> = HashMap::new();

    nums2.iter().enumerate().for_each(|(idx, &v)| {
      m.entry(v)
        .and_modify(|x| x.push(idx as i32))
        .or_insert(vec![idx as i32]);
    });

    fn dfs(
      nums1: &Vec<i32>,
      s1_idx: i32,
      s2_idx: i32,
      m: &HashMap<i32, Vec<i32>>,
      cache: &mut HashMap<i32, HashMap<i32, i32>>,
    ) -> i32 {
      if cache.contains_key(&s1_idx) && cache.get(&s1_idx).unwrap().contains_key(&s2_idx) {
        return *cache.get(&s1_idx).unwrap().get(&s2_idx).unwrap();
      }
      if s1_idx as usize == nums1.len() {
        return 0;
      }

      let mut cnt: i32 = 0;
      match m.get(&nums1[s1_idx as usize]) {
        Some(candi) => {
          let mut n_idx: usize = 0;
          while n_idx < candi.len() {
            if candi[n_idx] > s2_idx {
              cnt = cnt.max(dfs(nums1, s1_idx + 1, candi[n_idx], m, cache) + 1);
              break;
            }
            n_idx += 1;
          }
        }
        _ => {}
      };
      cnt = cnt.max(dfs(nums1, s1_idx + 1, s2_idx, m, cache));
      if !cache.contains_key(&s1_idx) {
        cache.insert(s1_idx, HashMap::new());
      }
      let v: &mut HashMap<i32, i32> = cache.get_mut(&s1_idx).unwrap();
      v.entry(s2_idx).and_modify(|x| *x = cnt).or_insert(cnt);
      cnt
    }

    // let mut max: i32 = 0;
    dfs(&nums1, 0, -1, &m, &mut cache);
    *cache.get(&0).unwrap().get(&-1).unwrap()
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4])
  );
  println!(
    "{}",
    Solution::max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2])
  );
  println!(
    "{}",
    Solution::max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1])
  );
}
