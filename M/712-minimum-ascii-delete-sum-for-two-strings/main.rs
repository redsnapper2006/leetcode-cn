use std::collections::HashMap;

impl Solution {
  pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let b1 = s1.as_bytes().to_vec();
    let b2 = s2.as_bytes().to_vec();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; b2.len() + 1]; b1.len() + 1];
    for i in 1..=b1.len() {
      dp[i][0] = dp[i - 1][0] + b1[i - 1] as i32;
    }
    for i in 1..=b2.len() {
      dp[0][i] = dp[0][i - 1] + b2[i - 1] as i32;
    }
    for i1 in 1..=b1.len() {
      for i2 in 1..=b2.len() {
        dp[i1][i2] = if b1[i1 - 1] == b2[i2 - 1] {
          dp[i1 - 1][i2 - 1]
        } else {
          (dp[i1 - 1][i2] + b1[i1 - 1] as i32).min(dp[i1][i2 - 1] + b2[i2 - 1] as i32)
        }
      }
    }
    dp[b1.len()][b2.len()]
  }

  pub fn minimum_delete_sum2(s1: String, s2: String) -> i32 {
    let mut cache: HashMap<usize, HashMap<usize, i32>> = HashMap::new();

    let b1 = s1.as_bytes().to_vec();
    let b2 = s2.as_bytes().to_vec();

    fn dfs(
      b1: &Vec<u8>, b2: &Vec<u8>, i1: usize, i2: usize,
      cache: &mut HashMap<usize, HashMap<usize, i32>>,
    ) -> i32 {
      if cache.contains_key(&i1) && cache.get(&i1).unwrap().contains_key(&i2) {
        return *cache.get(&i1).unwrap().get(&i2).unwrap();
      }
      if b1.len() == i1 {
        let mut sum: i32 = 0;
        for i in i2..b2.len() {
          sum += b2[i] as i32;
        }
        cache.entry(i1).or_insert(HashMap::new()).insert(i2, sum);
        sum
      } else if b2.len() == i2 {
        let mut sum: i32 = 0;
        for i in i1..b1.len() {
          sum += b1[i] as i32;
        }
        cache.entry(i1).or_insert(HashMap::new()).insert(i2, sum);
        sum
      } else {
        let v = if b1[i1] != b2[i2] {
          (dfs(b1, b2, i1 + 1, i2, cache) + b1[i1] as i32)
            .min(dfs(b1, b2, i1, i2 + 1, cache) + b2[i2] as i32)
        } else {
          dfs(b1, b2, i1 + 1, i2 + 1, cache)
        };
        cache.entry(i1).or_insert(HashMap::new()).insert(i2, v);
        v
      }
    }
    dfs(&b1, &b2, 0, 0, &mut cache)
  }
}
