struct Solution {}

impl Solution {
  pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
    let mut buf: Vec<Vec<i32>> = vec![vec![-1; 2]; 8];

    queens.iter().for_each(|cord| {
      let xd = cord[0] - king[0];
      let yd = cord[1] - king[1];

      match (
        xd,
        yd,
        xd.abs() == yd.abs(),
        cord[0] > king[0],
        cord[1] > king[1],
      ) {
        (_, 0, _, false, _) => {
          if buf[0][0] == -1 || buf[0][0] < cord[0] {
            buf[0][0] = cord[0];
            buf[0][1] = cord[1];
          }
        }
        (_, _, true, false, true) => {
          if buf[1][0] == -1 || buf[1][0] < cord[0] {
            buf[1][0] = cord[0];
            buf[1][1] = cord[1];
          }
        }
        (0, _, _, _, true) => {
          if buf[2][0] == -1 || buf[2][1] > cord[1] {
            buf[2][0] = cord[0];
            buf[2][1] = cord[1];
          }
        }
        (_, _, true, true, true) => {
          if buf[3][0] == -1 || buf[3][0] > cord[0] {
            buf[3][0] = cord[0];
            buf[3][1] = cord[1];
          }
        }
        (_, 0, _, true, _) => {
          if buf[4][0] == -1 || buf[4][0] > cord[0] {
            buf[4][0] = cord[0];
            buf[4][1] = cord[1];
          }
        }
        (_, _, true, true, false) => {
          if buf[5][0] == -1 || buf[5][0] > cord[0] {
            buf[5][0] = cord[0];
            buf[5][1] = cord[1];
          }
        }
        (0, _, _, _, false) => {
          if buf[6][0] == -1 || buf[6][1] < cord[1] {
            buf[6][0] = cord[0];
            buf[6][1] = cord[1];
          }
        }
        (_, _, true, false, false) => {
          if buf[7][0] == -1 || buf[7][0] < cord[0] {
            buf[7][0] = cord[0];
            buf[7][1] = cord[1];
          }
        }
        (_, _, _, _, _) => {}
      };
    });

    buf
      .iter()
      .filter(|cord| cord[0] != -1)
      .map(|cord| vec![cord[0], cord[1]])
      .collect()
  }
}

fn main() {
  // println!(
  //   "{:?}",
  //   Solution::queens_attackthe_king(
  //     vec![
  //       vec![0, 1],
  //       vec![1, 0],
  //       vec![4, 0],
  //       vec![0, 4],
  //       vec![3, 3],
  //       vec![2, 4]
  //     ],
  //     vec![0, 0]
  //   )
  // );

  println!(
    "{:?}",
    Solution::queens_attackthe_king(
      vec![
        vec![5, 6],
        vec![7, 7],
        vec![2, 1],
        vec![0, 7],
        vec![1, 6],
        vec![5, 1],
        vec![3, 7],
        vec![0, 3],
        vec![4, 0],
        vec![1, 2],
        vec![6, 3],
        vec![5, 0],
        vec![0, 4],
        vec![2, 2],
        vec![1, 1],
        vec![6, 4],
        vec![5, 4],
        vec![0, 0],
        vec![2, 6],
        vec![4, 5],
        vec![5, 2],
        vec![1, 4],
        vec![7, 5],
        vec![2, 3],
        vec![0, 5],
        vec![4, 2],
        vec![1, 0],
        vec![2, 7],
        vec![0, 1],
        vec![4, 6],
        vec![6, 1],
        vec![0, 6],
        vec![4, 3],
        vec![1, 7]
      ],
      vec![3, 4]
    )
  );
}
