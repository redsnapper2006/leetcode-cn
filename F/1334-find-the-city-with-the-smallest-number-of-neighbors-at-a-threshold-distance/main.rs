impl Solution {
  pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
    let n = n as usize;
    let mut floyd: Vec<Vec<i32>> = vec![vec![distance_threshold + 1; n]; n];

    edges.iter().for_each(|edge| {
      let e1 = edge[0] as usize;
      let e2 = edge[1] as usize;
      floyd[e1][e2] = edge[2];
      floyd[e2][e1] = edge[2];
    });

    for i in 0..n {
      floyd[i][i] = 0;

      for m in 0..n {
        if m == i || floyd[m][i] == distance_threshold {
          continue;
        }

        for n in 0..n {
          if n == i
            || n == m
            || floyd[i][n] == distance_threshold
            || floyd[m][i] + floyd[i][n] > distance_threshold
            || floyd[m][i] + floyd[i][n] >= floyd[m][n]
          {
            continue;
          }
          floyd[m][n] = floyd[m][i] + floyd[i][n];
          floyd[n][m] = floyd[m][i] + floyd[i][n];
        }
      }
    }

    let mut ans: usize = n;
    let mut mx = n + 1;
    for i in 0..n {
      let cnt = floyd[i].iter().filter(|&&x| x <= distance_threshold).count();
      if cnt < mx {
        ans = i;
        mx = cnt;
      } else if cnt == mx {
        ans = i;
      }
    }
    ans as _
  }
}
