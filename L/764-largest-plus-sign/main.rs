struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
    let mut rows: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut cols: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut mines_map: HashMap<i32, i32> = HashMap::new();

    for i in 0..mines.len() {
      let mine = &mines[i];
      mines_map.insert(mine[0] * n + mine[1], 1);

      let row = rows.entry(mine[0]).or_insert(Vec::new());
      row.push(mine[1]);
      let col = cols.entry(mine[1]).or_insert(Vec::new());
      col.push(mine[0]);
    }
    for (_, v) in rows.iter_mut() {
      (*v).sort()
    }
    for (_, v) in cols.iter_mut() {
      (*v).sort()
    }

    let mut ret: i32 = 0;
    for i in 0..n {
      for j in 0..n {
        if mines_map.contains_key(&(i * n + j)) {
          continue;
        }
        let mut left: i32 = 0;
        let mut right: i32 = 0;
        let mut top: i32 = 0;
        let mut bottom: i32 = 0;
        // , mut right, mut top, mut bottom): (i32, i32, i32, i32) = (0, 0, 0, 0);

        if !rows.contains_key(&i) {
          left = j;
          right = n - 1 - j;
        } else {
          let mut offset = 0;
          let row = rows.get(&i).unwrap();
          for m in (0..row.len()).rev() {
            if row[m] < j {
              offset = row[m] + 1;
              break;
            }
          }
          left = j - offset;
          offset = n - 1;
          for m in 0..row.len() {
            if row[m] > j {
              offset = row[m] - 1;
              break;
            }
          }
          right = offset - j;
        }

        if !cols.contains_key(&j) {
          top = i;
          bottom = n - 1 - i;
        } else {
          let mut offset = 0;
          let col = cols.get(&j).unwrap();
          for m in (0..col.len()).rev() {
            if col[m] < i {
              offset = col[m] + 1;
              break;
            }
          }
          top = i - offset;
          offset = n - 1;
          for m in 0..col.len() {
            if col[m] > i {
              offset = col[m] - 1;
              break;
            }
          }
          bottom = offset - i;
        }

        let mut val = n;
        if val > left {
          val = left;
        }
        if val > right {
          val = right;
        }
        if val > top {
          val = top;
        }
        if val > bottom {
          val = bottom;
        }
        if val + 1 > ret {
          ret = val + 1;
        }
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::order_of_largest_plus_sign(
      20,
      vec![
        vec![0, 0],
        vec![0, 1],
        vec![0, 2],
        vec![0, 3],
        vec![0, 4],
        vec![0, 5],
        vec![0, 7],
        vec![0, 9],
        vec![0, 11],
        vec![0, 13],
        vec![0, 14],
        vec![0, 15],
        vec![0, 16],
        vec![0, 17],
        vec![0, 18],
        vec![0, 19],
        vec![1, 0],
        vec![1, 1],
        vec![1, 5],
        vec![1, 6],
        vec![1, 7],
        vec![1, 9],
        vec![1, 10],
        vec![1, 11],
        vec![1, 12],
        vec![1, 13],
        vec![1, 14],
        vec![1, 15],
        vec![1, 18],
        vec![2, 0],
        vec![2, 1],
        vec![2, 3],
        vec![2, 4],
        vec![2, 5],
        vec![2, 9],
        vec![2, 10],
        vec![2, 11],
        vec![2, 15],
        vec![2, 16],
        vec![2, 18],
        vec![2, 19],
        vec![3, 1],
        vec![3, 2],
        vec![3, 3],
        vec![3, 5],
        vec![3, 7],
        vec![3, 8],
        vec![3, 9],
        vec![3, 10],
        vec![3, 11],
        vec![3, 12],
        vec![3, 13],
        vec![3, 14],
        vec![3, 15],
        vec![3, 17],
        vec![3, 19],
        vec![4, 0],
        vec![4, 1],
        vec![4, 2],
        vec![4, 3],
        vec![4, 4],
        vec![4, 5],
        vec![4, 6],
        vec![4, 7],
        vec![4, 9],
        vec![4, 10],
        vec![4, 13],
        vec![4, 14],
        vec![4, 15],
        vec![4, 17],
        vec![4, 18],
        vec![4, 19],
        vec![5, 0],
        vec![5, 1],
        vec![5, 4],
        vec![5, 5],
        vec![5, 8],
        vec![5, 10],
        vec![5, 12],
        vec![5, 13],
        vec![5, 14],
        vec![5, 15],
        vec![5, 16],
        vec![5, 17],
        vec![5, 18],
        vec![6, 1],
        vec![6, 2],
        vec![6, 4],
        vec![6, 5],
        vec![6, 6],
        vec![6, 7],
        vec![6, 8],
        vec![6, 11],
        vec![6, 13],
        vec![6, 14],
        vec![6, 15],
        vec![6, 16],
        vec![6, 18],
        vec![6, 19],
        vec![7, 0],
        vec![7, 1],
        vec![7, 2],
        vec![7, 3],
        vec![7, 4],
        vec![7, 6],
        vec![7, 7],
        vec![7, 8],
        vec![7, 10],
        vec![7, 11],
        vec![7, 12],
        vec![7, 13],
        vec![7, 14],
        vec![7, 16],
        vec![7, 17],
        vec![7, 19],
        vec![8, 0],
        vec![8, 1],
        vec![8, 5],
        vec![8, 6],
        vec![8, 7],
        vec![8, 8],
        vec![8, 9],
        vec![8, 11],
        vec![8, 13],
        vec![8, 15],
        vec![8, 16],
        vec![8, 17],
        vec![8, 18],
        vec![8, 19],
        vec![9, 2],
        vec![9, 3],
        vec![9, 4],
        vec![9, 5],
        vec![9, 6],
        vec![9, 7],
        vec![9, 8],
        vec![9, 9],
        vec![9, 10],
        vec![9, 11],
        vec![9, 12],
        vec![9, 14],
        vec![9, 15],
        vec![9, 16],
        vec![9, 17],
        vec![9, 18],
        vec![9, 19],
        vec![10, 1],
        vec![10, 3],
        vec![10, 4],
        vec![10, 6],
        vec![10, 7],
        vec![10, 8],
        vec![10, 9],
        vec![10, 10],
        vec![10, 13],
        vec![10, 14],
        vec![10, 16],
        vec![10, 17],
        vec![10, 19],
        vec![11, 0],
        vec![11, 1],
        vec![11, 2],
        vec![11, 4],
        vec![11, 5],
        vec![11, 7],
        vec![11, 8],
        vec![11, 10],
        vec![11, 11],
        vec![11, 13],
        vec![11, 14],
        vec![11, 15],
        vec![11, 16],
        vec![11, 17],
        vec![11, 18],
        vec![11, 19],
        vec![12, 0],
        vec![12, 1],
        vec![12, 3],
        vec![12, 4],
        vec![12, 5],
        vec![12, 7],
        vec![12, 8],
        vec![12, 9],
        vec![12, 10],
        vec![12, 11],
        vec![12, 12],
        vec![12, 13],
        vec![12, 14],
        vec![12, 17],
        vec![12, 18],
        vec![13, 0],
        vec![13, 1],
        vec![13, 2],
        vec![13, 3],
        vec![13, 5],
        vec![13, 6],
        vec![13, 7],
        vec![13, 8],
        vec![13, 9],
        vec![13, 10],
        vec![13, 11],
        vec![13, 12],
        vec![13, 17],
        vec![13, 18],
        vec![14, 0],
        vec![14, 2],
        vec![14, 3],
        vec![14, 4],
        vec![14, 5],
        vec![14, 7],
        vec![14, 9],
        vec![14, 11],
        vec![14, 12],
        vec![14, 14],
        vec![14, 15],
        vec![14, 17],
        vec![14, 19],
        vec![15, 2],
        vec![15, 3],
        vec![15, 5],
        vec![15, 7],
        vec![15, 8],
        vec![15, 9],
        vec![15, 11],
        vec![15, 13],
        vec![15, 14],
        vec![15, 15],
        vec![15, 18],
        vec![15, 19],
        vec![16, 0],
        vec![16, 1],
        vec![16, 2],
        vec![16, 4],
        vec![16, 7],
        vec![16, 8],
        vec![16, 9],
        vec![16, 10],
        vec![16, 11],
        vec![16, 12],
        vec![16, 16],
        vec![16, 17],
        vec![16, 18],
        vec![17, 1],
        vec![17, 3],
        vec![17, 4],
        vec![17, 5],
        vec![17, 6],
        vec![17, 10],
        vec![17, 11],
        vec![17, 13],
        vec![17, 16],
        vec![17, 17],
        vec![17, 19],
        vec![18, 0],
        vec![18, 1],
        vec![18, 4],
        vec![18, 6],
        vec![18, 9],
        vec![18, 10],
        vec![18, 11],
        vec![18, 13],
        vec![18, 14],
        vec![18, 15],
        vec![18, 16],
        vec![18, 17],
        vec![18, 19],
        vec![19, 0],
        vec![19, 1],
        vec![19, 2],
        vec![19, 3],
        vec![19, 4],
        vec![19, 6],
        vec![19, 7],
        vec![19, 8],
        vec![19, 9],
        vec![19, 10],
        vec![19, 11],
        vec![19, 12],
        vec![19, 14],
        vec![19, 15],
        vec![19, 16],
        vec![19, 17]
      ]
    )
  );
}
