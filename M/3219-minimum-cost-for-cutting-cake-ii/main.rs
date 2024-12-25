struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn minimum_cost(_m: i32, _n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i64 {
    let mut hc = horizontal_cut;
    let mut vc = vertical_cut;
    hc.sort_unstable();
    vc.sort_unstable();

    let mut hidx: i32 = hc.len() as i32 - 1;
    let mut vidx: i32 = vc.len() as i32 - 1;
    let mut hsegment: i64 = 1;
    let mut vsegment: i64 = 1;
    let mut ans: i64 = 0;
    while hidx >= 0 || vidx >= 0 {
      if hidx >= 0 && vidx >= 0 {
        if hc[hidx as usize] >= vc[vidx as usize] {
          ans += (hc[hidx as usize] as i64) * vsegment;
          hidx -= 1;
          hsegment += 1;
        } else {
          ans += (vc[vidx as usize] as i64) * hsegment;
          vidx -= 1;
          vsegment += 1;
        }
      } else if hidx >= 0 {
        ans += (hc[hidx as usize] as i64) * vsegment;
        hidx -= 1;
      } else {
        ans += (vc[vidx as usize] as i64) * hsegment;
        vidx -= 1;
      }
    }
    ans
  }

  pub fn minimum_cost2(m: i32, n: i32, horizontal_cut: Vec<i32>, vertical_cut: Vec<i32>) -> i32 {
    let mut cache: HashMap<(usize, usize, usize, usize), i32> = HashMap::new();

    fn dfs(
      top: usize,
      left: usize,
      bottom: usize,
      right: usize,
      hc: &Vec<i32>,
      vc: &Vec<i32>,
      cache: &mut HashMap<(usize, usize, usize, usize), i32>,
    ) -> i32 {
      if cache.contains_key(&(top, left, bottom, right)) {
        return *cache.get(&(top, left, bottom, right)).unwrap();
      }
      if bottom - top == 1 && right - left == 1 {
        cache.insert((top, left, bottom, right), 0);
        return 0;
      }

      let mut min: i32 = i32::MAX;
      (top + 1..bottom).for_each(|row| {
        min = min.min(
          hc[row - 1]
            + dfs(top, left, row, right, hc, vc, cache)
            + dfs(row, left, bottom, right, hc, vc, cache),
        );
      });
      (left + 1..right).for_each(|col| {
        min = min.min(
          vc[col - 1]
            + dfs(top, left, bottom, col, hc, vc, cache)
            + dfs(top, col, bottom, right, hc, vc, cache),
        );
      });

      cache.insert((top, left, bottom, right), min);
      min
    }

    dfs(
      0,
      0,
      m as usize,
      n as usize,
      &horizontal_cut,
      &vertical_cut,
      &mut cache,
    )
  }
}

fn main() {
  println!("{}", Solution::minimum_cost(3, 2, vec![1, 3], vec![5]));
  println!("{}", Solution::minimum_cost2(2, 2, vec![7], vec![4]));
}
