struct Solution {}

impl Solution {
  pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let mm: Vec<Vec<i32>> = vec![
      vec![0],
      vec![1],
      vec![2, 4, 8, 6],
      vec![3, 9, 7, 1],
      vec![4, 6],
      vec![5],
      vec![6],
      vec![7, 9, 3, 1],
      vec![8, 4, 2, 6],
      vec![9, 1],
    ];

    variables.iter().enumerate().for_each(|(idx, var)| {
      let mut a = var[0] % 10;
      let b = (var[1] - 1) % (mm[a as usize].len() as i32);
      a = mm[a as usize][b as usize] % var[3];

      let mut mmm: Vec<i32> = Vec::new();
      let mut i: i32 = 0;
      let mut aggr: i32 = a;
      while i < var[2] {
        mmm.push(aggr);
        aggr *= a;
        aggr %= var[3];
        if aggr == mmm[0] {
          break;
        }
        i += 1;
      }
      let mut n_idx: usize = 0;
      if var[2] == i {
        n_idx = mmm.len() - 1;
      } else {
        n_idx = ((var[2] - 1) % (mmm.len() as i32)) as usize;
      }

      if mmm[n_idx] == target {
        ans.push(idx as i32);
      }
    });

    ans
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::get_good_indices(
      vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]],
      2
    )
  );
  println!(
    "{:?}",
    Solution::get_good_indices(vec![vec![39, 3, 1000, 1000]], 17)
  );
}
