impl Solution {
  pub fn path_existence_queries(
    n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>,
  ) -> Vec<i32> {
    let m = n.ilog2() as usize + 1;
    let mut sorted_nums = nums;
    sorted_nums.sort_unstable();

    let mut idx: Vec<usize> = (0..n as usize).collect();
    idx.sort_by_key(|&i| nums[i]);

    let mut pos = vec![0; n as usize];
    for (i, &v) in idx.iter().enumerate() {
      pos[v] = i;
    }

    let mut jump: Vec<Vec<usize>> = vec![vec![0; m]; n as usize];
    let mut step: usize = 0;
    for i in 1..sorted_nums.len() {
      while step < i && sorted_nums[i] - sorted_nums[step] > max_diff {
        step += 1;
      }
      jump[i][0] = step;
    }
    for j in 1..m {
      for i in 0..sorted_nums.len() {
        jump[i][j] = jump[jump[i][j - 1]][j - 1];
      }
    }

    queries
      .iter()
      .map(|q| {
        let mut idx0 = pos[q[0] as usize];
        let mut idx1 = pos[q[1] as usize];

        if idx0 > idx1 {
          std::mem::swap(&mut idx0, &mut idx1);
        }

        if idx0 == idx1 {
          0
        } else {
          let mut cnt: i32 = 0;
          for j in (0..m).rev() {
            if jump[idx1][j] > idx0 {
              idx1 = jump[idx1][j];
              cnt += 1 << j;
            }
          }
          if jump[idx1][0] <= idx0 { cnt + 1 } else { -1 }
        }
      })
      .collect::<Vec<i32>>()
  }
}
