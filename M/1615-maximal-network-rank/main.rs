struct Solution {}

impl Solution {
  pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut gress: [[i32; 101]; 101] = [[0; 101]; 101];
    roads.iter().for_each(|edge| {
      let (e1, e2) = (edge[0], edge[1]);
      gress[e1 as usize][e2 as usize] = 1;
      gress[e2 as usize][e1 as usize] = 1;
    });

    let mut sum_gress: [i32; 101] = [0; 101];
    (0..n).for_each(|row| {
      let mut sum: i32 = 0;
      (0..n).for_each(|col| {
        sum += gress[row as usize][col as usize];
      });
      sum_gress[row as usize] = sum;
    });

    let mut ret: i32 = 0;
    (0..n).for_each(|row| {
      (0..n).for_each(|col| {
        if row != col
          && ret
            < sum_gress[row as usize] + sum_gress[col as usize] - gress[row as usize][col as usize]
        {
          ret =
            sum_gress[row as usize] + sum_gress[col as usize] - gress[row as usize][col as usize];
        }
      });
    });

    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::maximal_network_rank(
      8,
      vec![
        vec![0, 1],
        vec![1, 2],
        vec![2, 3],
        vec![2, 4],
        vec![5, 6],
        vec![5, 7]
      ]
    )
  );
}
