use std::collections::HashMap;

impl Solution {
  pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
    let mut edge: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, &p) in parent.iter().enumerate() {
      if p == -1 {
        continue;
      }
      edge.entry(p as usize).or_insert(vec![]).push(i);
    }
    let n = parent.len();
    let colors = s.as_bytes().into_iter().map(|b| (b - b'a') as usize).collect::<Vec<usize>>();
    let mut ans: Vec<i32> = vec![0; n];

    fn dfs(
      edge: &HashMap<usize, Vec<usize>>, cur: usize, colors: &[usize], prev: &Vec<i32>,
      ans: &mut [i32],
    ) -> i32 {
      if !edge.contains_key(&cur) {
        ans[cur] = 1;
        if cur != 0 && prev[colors[cur]] != -1 {
          ans[prev[colors[cur]] as usize] += 1;
          return 0;
        }
        return 1;
      }

      let mut prev2 = prev.clone();
      prev2[colors[cur]] = cur as i32;
      let mut sum = 0i32;
      for &nxt in edge.get(&cur).unwrap() {
        let ret = dfs(edge, nxt, colors, &prev2, ans);
        if prev[colors[nxt]] != -1 && colors[nxt] != colors[cur] {
          continue;
        }

        sum += ret;
      }

      ans[cur] += sum + 1;
      if cur != 0 && prev[colors[cur]] != -1 {
        ans[prev[colors[cur]] as usize] += ans[cur];
        return 0;
      }
      ans[cur]
    }
    let mut prev: Vec<i32> = vec![-1; 26];
    prev[colors[0]] = 0;
    dfs(&edge, 0, &colors, &prev, &mut ans);
    ans
  }
}
