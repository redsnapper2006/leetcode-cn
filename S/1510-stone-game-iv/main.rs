impl Solution {
  pub fn winner_square_game(n: i32) -> bool {
    let mut cache: std::collections::HashMap<i32, bool> = std::collections::HashMap::new();
    cache.insert(0, false);
    fn dfs(n: i32, cache: &mut std::collections::HashMap<i32, bool>) -> bool {
      if let Some(v) = cache.get(&n) {
        return *v;
      }

      let mut ret: bool = false;
      for i in 1..=n {
        if i * i > n {
          break;
        }
        if !dfs(n - i * i, cache) {
          ret = true;
          break;
        }
      }
      cache.insert(n, ret);
      ret
    }

    dfs(n, &mut cache)
  }

  pub fn winner_square_game2(n: i32) -> bool {
    let n = n as usize;
    let mut dp: Vec<bool> = vec![false; n + 1];
    for i in (1..=n) {
      let mut j: usize = 1;
      while j * j <= i {
        if !dp[i - j * j] {
          dp[i] = true;
          break;
        }
        j += 1;
      }
    }
    dp[n]
  }
}
