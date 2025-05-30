
use std::collections::HashSet;
impl Solution {
  pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
    fn dfs(
      current: usize,
      step: i32,
      buf: &mut Vec<i32>,
      visit: &mut HashSet<usize>,
      edges: &Vec<i32>,
    ) {
      visit.insert(current);
      let next = edges[current];
      if next == -1 || visit.contains(&(next as usize)) {
        return;
      }
      let next = next as usize;

      buf[next] = step + 1;
      dfs(next, step + 1, buf, visit, edges);
    }
    let mut buf1: Vec<i32> = vec![-1; edges.len()];
    let mut visit1: HashSet<usize> = HashSet::new();
    let node1 = node1 as usize;
    buf1[node1] = 0;
    dfs(node1, 0, &mut buf1, &mut visit1, &edges);

    let mut buf2: Vec<i32> = vec![-1; edges.len()];
    let mut visit2: HashSet<usize> = HashSet::new();
    let node2 = node2 as usize;
    buf2[node2] = 0;
    dfs(node2, 0, &mut buf2, &mut visit2, &edges);

    // println!("1 {:?}", buf1);
    // println!("2 {:?}", buf2);
    let mut ans : i32 = -1;
    let mut step : i32 = edges.len() as i32;
    (0..edges.len()).for_each(|idx|{
      if buf1[idx] == -1 || buf2[idx] == -1 {
        return;
      }
      let v = buf1[idx].max(buf2[idx]);
      if v < step {
        step = v;
        ans = idx as i32;
      }
    });
    ans
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::closest_meeting_node(vec![2, 2, 3, -1], 0, 1)
  );
  println!("{:?}", Solution::closest_meeting_node(vec![1, 2, -1], 0, 2));
}
