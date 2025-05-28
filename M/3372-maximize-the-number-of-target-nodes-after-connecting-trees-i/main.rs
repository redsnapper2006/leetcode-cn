use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
  pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut m1: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut m2: HashMap<usize, Vec<usize>> = HashMap::new();

    fn build(edges: &Vec<Vec<i32>>, m: &mut HashMap<usize, Vec<usize>>) {
      edges.iter().for_each(|edge| {
        let e0 = edge[0] as usize;
        let e1 = edge[1] as usize;
        m.entry(e0).and_modify(|x| x.push(e1)).or_insert(vec![e1]);
        m.entry(e1).and_modify(|x| x.push(e0)).or_insert(vec![e0]);
      });
    }
    build(&edges1, &mut m1);
    build(&edges2, &mut m2);

    fn dfs(
      current: usize,
      buf: &mut Vec<Vec<i32>>,
      m: &HashMap<usize, Vec<usize>>,
      visit: &mut HashSet<usize>,
    ) {
      visit.insert(current);
      let next = m.get(&current).unwrap();
      for n in next {
        if visit.contains(&n) {
          continue;
        }
        (0..buf[current].len()).for_each(|idx| {
          if buf[current][idx] == 0 {
            return;
          }
          buf[*n][idx] = buf[current][idx] + 1;
          buf[idx][*n] = buf[*n][idx];
        });
        buf[current][*n] = 1;
        buf[*n][current] = 1;
        dfs(*n, buf, m, visit);
      }
    }
    let mut buf1: Vec<Vec<i32>> = vec![vec![0; edges1.len() + 1]; edges1.len() + 1];
    let mut visit1: HashSet<usize> = HashSet::new();
    dfs(0, &mut buf1, &m1, &mut visit1);

    let mut buf2: Vec<Vec<i32>> = vec![vec![0; edges2.len() + 1]; edges2.len() + 1];
    let mut visit2: HashSet<usize> = HashSet::new();
    dfs(0, &mut buf2, &m2, &mut visit2);

    for b1 in &mut buf1 {
      b1.sort_unstable();
    }
    let mut b2_max: i32 = 0;
    for b2 in &mut buf2 {
      b2.sort_unstable();
      b2_max = b2_max.max(b2.partition_point(|x| *x <= (k - 1)) as i32);
    }

    (0..edges1.len() + 1)
      .map(|idx| buf1[idx].partition_point(|x| *x <= k) as i32 + b2_max)
      .collect::<Vec<i32>>()
  }
}

struct Solution {}

fn main() {
  println!(
    "{:?}",
    Solution::max_target_nodes(
      vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
      vec![
        vec![0, 1],
        vec![0, 2],
        vec![0, 3],
        vec![2, 7],
        vec![1, 4],
        vec![4, 5],
        vec![4, 6]
      ],
      2
    )
  );

  println!(
    "{:?}",
    Solution::max_target_nodes(
      vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
      vec![vec![0, 1], vec![1, 2], vec![2, 3]],
      1
    )
  );
}
