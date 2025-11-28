impl Solution {
  pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
    let mut m: Vec<Vec<i32>> = vec![vec![]; n as usize];
    edges.iter().for_each(|edge| {
      m[edge[0] as usize].push(edge[1]);
      m[edge[1] as usize].push(edge[0]);
    });

    let mut ans: i32 = 0;
    fn dfs(
      cur: i32, parent: i32, m: &Vec<Vec<i32>>, values: &Vec<i32>, k: i32, ans: &mut i32,
    ) -> i32 {
      let mut sum: i32 = values[cur as usize];
      for &nxt in &m[cur as usize] {
        if nxt == parent {
          continue;
        }
        sum += dfs(nxt, cur, m, values, k, ans);
      }
      if sum % k == 0 {
        *ans += 1;
      }
      sum % k
    }
    dfs(0, -1, &m, &values, k, &mut ans);
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::max_k_divisible_components(
      5,
      vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
      vec![1, 8, 1, 4, 4],
      6
    )
  );

  println!(
    "{}",
    Solution::max_k_divisible_components(
      7,
      vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 3],
        vec![1, 4],
        vec![2, 5],
        vec![2, 6]
      ],
      vec![3, 0, 6, 1, 5, 2, 1],
      3
    )
  );
}
