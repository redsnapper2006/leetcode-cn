struct Solution {}

impl Solution {
  pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    let mut rooms = rooms;
    rooms.sort_by(|x, y| {
      if x[1] == y[1] {
        x[0].cmp(&y[0])
      } else {
        x[1].cmp(&y[1])
      }
    });

    let mut q2 = queries
      .into_iter()
      .enumerate()
      .map(|x| x)
      .collect::<Vec<(usize, Vec<i32>)>>();
    q2.sort_by(|x, y| {
      if x.1[1] == y.1[1] {
        x.1[0].cmp(&y.1[0])
      } else {
        x.1[1].cmp(&y.1[1])
      }
    });

    let mut offset: Vec<i32> = Vec::new();
    let mut idx1: i32 = q2.len() as i32 - 1;
    let mut idx2: i32 = rooms.len() as i32 - 1;
    let mut ans: Vec<i32> = vec![-1; q2.len()];
    while idx1 >= 0 {
      while idx2 >= 0 && rooms[idx2 as usize][1] >= q2[idx1 as usize].1[1] {
        let ll = match offset.binary_search_by(|p| p.cmp(&rooms[idx2 as usize][0])) {
          Ok(ov) => ov,
          Err(ev) => ev,
        };
        offset.insert(ll, rooms[idx2 as usize][0]);
        idx2 -= 1;
      }

      let ll2 = match offset.binary_search_by(|p| p.cmp(&q2[idx1 as usize].1[0])) {
        Ok(ov) => ov,
        Err(ev) => ev,
      };
      let mut ret: i32 = -1;
      if offset.len() > 0 {
        if ll2 == offset.len() {
          ret = offset[offset.len() - 1];
        } else if ll2 == 0 {
          ret = offset[0];
        } else {
          ret = if (offset[ll2 - 1] - q2[idx1 as usize].1[0]).abs()
            > (offset[ll2] - q2[idx1 as usize].1[0]).abs()
          {
            offset[ll2]
          } else {
            offset[ll2 - 1]
          };
        }
      }
      ans[q2[idx1 as usize].0] = ret;
      idx1 -= 1;
    }

    ans
  }
}

fn main() {
  println!(
    "{:?}",
    Solution::closest_room(
      vec![vec![2, 2], vec![1, 2], vec![3, 2]],
      vec![vec![3, 1], vec![3, 3], vec![5, 2]]
    )
  );

  println!(
    "{:?}",
    Solution::closest_room(
      vec![vec![1, 4], vec![2, 3], vec![3, 5], vec![4, 1], vec![5, 2]],
      vec![vec![2, 3], vec![2, 4], vec![2, 5]]
    )
  );

  println!(
    "{:?}",
    Solution::closest_room(
      vec![
        vec![23, 22],
        vec![6, 20],
        vec![15, 6],
        vec![22, 19],
        vec![2, 10],
        vec![21, 4],
        vec![10, 18],
        vec![16, 1],
        vec![12, 7],
        vec![5, 22]
      ],
      vec![
        vec![12, 5],
        vec![15, 15],
        vec![21, 6],
        vec![15, 1],
        vec![23, 4],
        vec![15, 11],
        vec![1, 24],
        vec![3, 19],
        vec![25, 8],
        vec![18, 6]
      ]
    )
  );
}
