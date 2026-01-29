impl Solution {
  pub fn minimum_cost(
    source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>,
  ) -> i64 {
    let mut floyd: Vec<Vec<i64>> = vec![vec![i64::MAX; 26]; 26];
    for i in 0..26 {
      floyd[i][i] = 0;
    }

    for i in 0..original.len() {
      let s = original[i] as usize - 'a' as usize;
      let d = changed[i] as usize - 'a' as usize;
      floyd[s][d] = floyd[s][d].min(cost[i] as i64);
    }

    for i in 0..26 {
      for j in 0..26 {
        for k in 0..26 {
          if i == j || i == k || floyd[j][i] == i64::MAX || floyd[i][k] == i64::MAX {
            continue;
          }
          floyd[j][k] = floyd[j][k].min(floyd[j][i] + floyd[i][k]);
        }
      }
    }

    let bbs = source.as_bytes().to_vec();
    let bbt = target.as_bytes().to_vec();
    let mut ans: i64 = 0;
    for i in 0..bbs.len() {
      if floyd[(bbs[i] - b'a') as usize][(bbt[i] - b'a') as usize] == i64::MAX {
        return -1;
      }
      ans += floyd[(bbs[i] - b'a') as usize][(bbt[i] - b'a') as usize];
    }
    ans
  }
}
