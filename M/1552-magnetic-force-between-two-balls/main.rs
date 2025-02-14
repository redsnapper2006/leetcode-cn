struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
    let mut position = position;
    position.sort_unstable();
    let min = position[0];
    let max = position[position.len() - 1];

    if m == 2 {
      return max - min;
    }

    let mut start: i32 = 1;
    let mut end: i32 = max - min;
    while start <= end {
      let step = start + (end - start) / 2;

      let mut cnt: i32 = 1;
      let mut base = min;
      while base < max {
        base += step;
        let e = match position.binary_search(&base) {
          Ok(ov) => ov,
          Err(ev) => ev,
        };
        if e >= position.len() {
          break;
        }
        cnt += 1;
        base = position[e];
      }
      if cnt < m {
        end = step - 1;
      } else {
        start = step + 1;
      }
    }
    end
  }

  #[allow(dead_code)]
  #[allow(non_snake_case)]
  pub fn max_distance2(position: Vec<i32>, m: i32) -> i32 {
    let mut position = position;
    position.sort_unstable();

    if m == 2 {
      return position[position.len() - 1] - position[0];
    }

    let mut cache: HashMap<usize, HashMap<i32, i32>> = HashMap::new();
    fn dfs(
      position: &Vec<i32>,
      idx: usize,
      m: i32,
      cache: &mut HashMap<usize, HashMap<i32, i32>>,
    ) -> i32 {
      if cache.contains_key(&idx) && cache.get(&idx).unwrap().contains_key(&m) {
        return *cache.get(&idx).unwrap().get(&m).unwrap();
      }

      let N = 1000000001;
      if m == 0 {
        cache
          .entry(idx)
          .and_modify(|x| {
            let _ = *x.entry(m).and_modify(|y| *y = N).or_insert(N);
          })
          .or_insert(HashMap::from([(0, N)]));
        return N;
      }

      let mut base: i32 = 0;
      (idx + 1..position.len()).for_each(|ii| {
        if ((position.len() - ii) as i32) < m {
          return;
        }
        let mut v = dfs(position, ii, m - 1, cache);
        v = v.min(position[ii] - position[idx]);
        base = base.max(v);
      });
      cache
        .entry(idx)
        .and_modify(|x| {
          let _ = *x.entry(m).and_modify(|y| *y = base).or_insert(base);
        })
        .or_insert(HashMap::from([(m, base)]));

      base
    }
    dfs(&position, 0, m - 1, &mut cache);
    // println!("{:?}", cache);
    *cache.get(&0).unwrap().get(&(m - 1)).unwrap()
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_distance(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4)
  );
}
