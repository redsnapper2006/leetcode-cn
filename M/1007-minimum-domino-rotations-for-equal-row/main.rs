struct Solution {}

impl Solution {
  pub fn min_domino_rotations(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut idx: usize = 0;
    let mut at_cnt: i32 = 0;
    let mut ab_cnt: i32 = 0;
    let mut bt_cnt: i32 = 0;
    let mut bb_cnt: i32 = 0;
    while idx < a.len() {
      if at_cnt >= 0 {
        if a[idx] != a[0] && b[idx] != a[0] {
          at_cnt = -1;
        } else if a[idx] != a[0] {
          at_cnt += 1;
        } else if b[idx] != a[0] {
          ab_cnt += 1;
        }
      }
      if bt_cnt >= 0 {
        if b[idx] != b[0] && a[idx] != b[0] {
          bt_cnt = -1;
        } else if b[idx] != b[0] {
          bt_cnt += 1;
        } else if a[idx] != b[0] {
          bb_cnt += 1;
        }
      }
      idx += 1;
    }
    match (at_cnt, bt_cnt) {
      (-1, -1) => -1,
      (-1, _) => bt_cnt.min(bb_cnt),
      (_, -1) => at_cnt.min(ab_cnt),
      (_, _) => at_cnt.min(ab_cnt).min(bt_cnt).min(bb_cnt),
    }
  }

  pub fn min_domino_rotations2(a: Vec<i32>, b: Vec<i32>) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; 6]; a.len()];
    for i in 0..a.len() {
      buf[i][a[i] as usize - 1] += 1;
      buf[i][b[i] as usize - 1] += 1;
    }
    // println!("{:?}", buf);
    let mut candi: Vec<i32> = Vec::new();
    for i in 0..6 {
      let mut cnt: i32 = 0;
      for j in 0..a.len() {
        if buf[j][i] >= 1 {
          cnt += 1;
        }
      }
      if cnt >= a.len() as i32 {
        candi.push(i as i32 + 1);
      }
    }
    if candi.len() == 0 {
      return -1;
    }
    // println!("{:?}", candi);
    let mut ret: i32 = a.len() as i32;
    for i in 0..candi.len() {
      let c = candi[i];
      let mut am: i32 = 0;
      let mut bm: i32 = 0;
      for j in 0..a.len() {
        if a[j] != c {
          am += 1;
        }
        if b[j] != c {
          bm += 1;
        }
      }
      if ret > am {
        ret = am;
      }
      if ret > bm {
        ret = bm;
      }
    }
    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2])
  );

  println!(
    "{}",
    Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4])
  );
}
