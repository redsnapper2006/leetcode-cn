impl Solution {
  pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut q: Vec<Vec<i32>> = vec![vec![0]];

    let mut ans: Vec<Vec<i32>> = vec![];
    while q.len() > 0 {
      let cur = q.pop().unwrap();

      for nxt in &graph[cur[cur.len() - 1] as usize] {
        let mut t = cur.clone();
        t.push(*nxt);
        if *nxt == graph.len() as i32 - 1 {
          ans.push(t);
        } else {
          q.push(t);
        }
      }
    }

    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]])
  );

  println!(
    "{:?}",
    Solution::all_paths_source_target(vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]])
  );
}
