struct Solution {}

impl Solution {
  pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut candidates = candidates;
    candidates.sort();
    let mut dp: Vec<Vec<Vec<Vec<i32>>>> =
      vec![vec![Vec::new(); candidates.len()]; target as usize + 1];

    candidates.iter().enumerate().for_each(|(i, &v)| {
      if v <= target {
        dp[v as usize][i].push(vec![v]);
      }
    });
    (0..target as usize).for_each(|base| {
      let mut idx: usize = 0;
      while idx < candidates.len() {
        if dp[base][idx].len() == 0 {
          idx += 1;
          continue;
        }

        let mut idx2: usize = idx;

        while idx2 < candidates.len() {
          if base as i32 + candidates[idx2] > target {
            break;
          }

          let mut idx3: usize = 0;
          while idx3 < dp[base][idx].len() {
            let mut vv = dp[base][idx][idx3].clone();
            vv.push(candidates[idx2]);
            dp[base + candidates[idx2] as usize][idx2].push(vv);
            idx3 += 1;
          }

          idx2 += 1;
        }
        idx += 1;
      }
    });

    let mut ret: Vec<Vec<i32>> = Vec::new();
    dp[target as usize].iter().for_each(|vvv| {
      vvv.iter().for_each(|vv| {
        ret.push(vv.clone());
      });
    });
    ret
  }
}

fn main() {
  println!("{:?}", Solution::combination_sum(vec![2, 3, 6, 7], 7));
}
