struct Solution {}

impl Solution {
  pub fn recur(idx: i32, pidx: i32, ee: &Vec<Vec<i32>>, label: &Vec<u8>, ret: &mut Vec<Vec<i32>>) {
    let c = &ee[idx as usize];
    let mut r: Vec<i32> = vec![0; 26];

    for i in 0..c.len() {
      if c[i] == pidx {
        continue;
      }
      if ret[c[i] as usize].len() == 0 {
        Solution::recur(c[i], idx, ee, label, ret);
      }
      let ll = &ret[c[i] as usize];
      for i in 0..26 {
        r[i] += ll[i];
      }
    }

    r[(label[idx as usize] - 'a' as u8) as usize] += 1;
    ret[idx as usize] = r;
  }

  pub fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32> {
    let ll: Vec<u8> = labels.as_bytes().to_vec();
    let mut ee: Vec<Vec<i32>> = vec![Vec::new(); n as usize];
    let mut edge_arr: Vec<Vec<i32>> = vec![Vec::new(); n as usize];

    for i in 0..edges.len() {
      let e = &edges[i];
      let p = e[0];
      let c = e[1];
      ee[p as usize].push(c);
      ee[c as usize].push(p);
    }

    Solution::recur(0, -1, &ee, &ll, &mut edge_arr);

    let mut ret: Vec<i32> = vec![0; n as usize];
    for i in 0..n {
      ret[i as usize] = edge_arr[i as usize][(ll[i as usize] - 'a' as u8) as usize];
    }
    ret
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::count_sub_trees(
      7,
      vec![
        vec![0, 1],
        vec![0, 2],
        vec![1, 4],
        vec![1, 5],
        vec![2, 3],
        vec![2, 6]
      ],
      String::from("abaedcd")
    )
  )
}
