struct Solution {}

impl Solution {
  pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
    fn dfs(cur: i32, arr: Vec<i32>, k: i32, n: i32, ret: &mut Vec<Vec<i32>>) {
      let mut a = arr.clone();
      a.push(cur);
      if arr.iter().sum::<i32>() > n {
        return;
      }

      if a.iter().sum::<i32>() == n {
        if a.len() == k as usize {
          ret.push(a);
        }
        return;
      }

      (cur + 1..10).for_each(|d| {
        dfs(d, a.clone(), k, n, ret);
      });
    }

    let mut ret: Vec<Vec<i32>> = Vec::new();
    (1..10).for_each(|d| {
      dfs(d, Vec::new(), k, n, &mut ret);
    });
    ret
  }
}
