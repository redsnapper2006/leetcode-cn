use std::collections::HashMap;
impl Solution {
  pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
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

    // black : 1 , white : 0
    fn dfs(
      current: usize,
      cur_color: i32,
      black: &mut i32,
      white: &mut i32,
      m: &HashMap<usize, Vec<usize>>,
      visit: &mut HashMap<usize, i32>,
    ) {
      visit.insert(current, cur_color);
      *black += if cur_color == 1 { 1 } else { 0 };
      *white += if cur_color == 0 { 1 } else { 0 };
      let next = m.get(&current).unwrap();
      for n in next {
        if visit.contains_key(&n) {
          continue;
        }
        dfs(*n, 1 - cur_color, black, white, m, visit);
      }
    }

    let mut black1: i32 = 0;
    let mut white1: i32 = 0;
    let mut visit1: HashMap<usize, i32> = HashMap::new();
    dfs(0, 1, &mut black1, &mut white1, &m1, &mut visit1);

    let mut black2: i32 = 0;
    let mut white2: i32 = 0;
    let mut visit2: HashMap<usize, i32> = HashMap::new();
    dfs(0, 1, &mut black2, &mut white2, &m2, &mut visit2);

    (0..edges1.len() + 1)
      .map(|idx| if *visit1.get(&idx).unwrap() == 1  { black1 } else { white1} + black2.max(white2))
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
      ]
    )
  );

  println!(
    "{:?}",
    Solution::max_target_nodes(
      vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4]],
      vec![vec![0, 1], vec![1, 2], vec![2, 3]],
    )
  );
}
