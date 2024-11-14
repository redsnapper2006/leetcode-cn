struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32 {
    let mut em: HashMap<i32, Vec<i32>> = HashMap::new();
    edges.iter().for_each(|edge| {
      let p = edge[0];
      let c = edge[1];
      em.entry(p).and_modify(|x| x.push(c)).or_insert(vec![c]);
      em.entry(c).and_modify(|x| x.push(p)).or_insert(vec![p]);
    });

    fn recur(p: i32, em: &HashMap<i32, Vec<i32>>, visited: &mut Vec<i32>, ans: &mut i32) -> i32 {
      visited[p as usize] = 1;

      let mut cs: Vec<i32> = Vec::new();
      em.get(&p).unwrap().iter().for_each(|&c| {
        if visited[c as usize] == 1 {
          return;
        }
        cs.push(recur(c, em, visited, ans));
      });
      if cs.len() == 0 {
        *ans += 1;
        return 1;
      }

      if cs.iter().all(|&x| x == cs[0]) {
        *ans += 1;
      }
      cs.iter().sum::<i32>() + 1
    }

    let mut ans: i32 = 0;
    let mut visited: Vec<i32> = vec![0; edges.len() + 1];
    recur(0, &em, &mut visited, &mut ans);
    ans
  }
}

fn main() {
  println!(
    "{}",
    Solution::count_good_nodes(vec![
      vec![0, 1],
      vec![1, 2],
      vec![1, 3],
      vec![1, 4],
      vec![0, 5],
      vec![5, 6],
      vec![6, 7],
      vec![7, 8],
      vec![0, 9],
      vec![9, 10],
      vec![9, 12],
      vec![10, 11]
    ])
  );

  println!(
    "{}",
    Solution::count_good_nodes(vec![
      vec![6, 0],
      vec![1, 0],
      vec![5, 1],
      vec![2, 5],
      vec![3, 1],
      vec![4, 3]
    ])
  );
}
