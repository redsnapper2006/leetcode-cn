use std::collections::HashMap;
impl Solution {
  pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut colors_set: Vec<usize> = (0..n).collect();

    fn find(x: usize, set: &mut Vec<usize>) -> usize {
      if set[x] != x {
        set[x] = find(set[x], set);
      }
      set[x]
    }

    fn union(x: usize, y: usize, set: &mut Vec<usize>) {
      let xx = find(x, set);
      let yy = find(y, set);
      if xx != yy {
        if xx < yy {
          set[xx] = yy;
        } else {
          set[yy] = xx;
        }
      }
    }

    edges.iter().for_each(|e| {
      union(e[0] as usize, e[1] as usize, &mut colors_set);
    });

    let mut node_cnt: Vec<i32> = vec![0; n];
    let mut edge_cnt: Vec<i32> = vec![0; n];
    (0..n).for_each(|i| {
      node_cnt[find(i, &mut colors_set)] += 1;
    });

    edges.iter().for_each(|e| {
      edge_cnt[find(e[0] as usize, &mut colors_set)] += 1;
    });

    (0..n).fold(0, |ans, i| {
      if colors_set[i] == i && node_cnt[i] * (node_cnt[i] - 1) == edge_cnt[i] * 2 {
        ans + 1
      } else {
        ans
      }
    })
  }

  pub fn count_complete_components2(n: i32, edges: Vec<Vec<i32>>) -> i32 {
    let mut cnt: Vec<i32> = vec![0; n as usize];
    let mut colors: Vec<i32> = vec![-1; n as usize];
    let mut m: Vec<Vec<i32>> = vec![vec![]; n as usize];

    edges.iter().for_each(|edge| {
      m[edge[0] as usize].push(edge[1]);
      cnt[edge[0] as usize] += 1;
      m[edge[1] as usize].push(edge[0]);
      cnt[edge[1] as usize] += 1;
    });

    fn dfs(cur: i32, m: &Vec<Vec<i32>>, colors: &mut Vec<i32>, color: i32) {
      colors[cur as usize] = color;

      for n in &m[cur as usize] {
        if colors[*n as usize] != -1 {
          continue;
        }
        dfs(*n, m, colors, color);
      }
    }

    let mut color_cnt: i32 = 0;
    for ii in 0..n {
      if colors[ii as usize] != -1 {
        continue;
      }
      color_cnt += 1;
      dfs(ii, &m, &mut colors, color_cnt);
    }

    // println!("{:?}", colors);

    let mut c_cnt: HashMap<i32, i32> = HashMap::new();
    for ii in 0..n {
      c_cnt.entry(colors[ii as usize]).and_modify(|x| *x += 1).or_insert(1);
    }

    // println!("{:?}", c_cnt);
    // println!("{:?}", cnt);

    for ii in 0..n {
      let v = colors[ii as usize];
      if !c_cnt.contains_key(&v) {
        continue;
      }
      if *c_cnt.get(&v).unwrap() != cnt[ii as usize] + 1 {
        c_cnt.remove(&v);
      }
    }

    c_cnt.len() as i32
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::count_complete_components(6, vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]])
  );

  println!(
    "{}",
    Solution::count_complete_components(
      6,
      vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
    )
  );
}
