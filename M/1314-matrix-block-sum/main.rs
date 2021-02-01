struct Solution {}

impl Solution {
  pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; mat[0].len()]; mat.len()];
    for i in 0..mat.len() {
      for j in 0..mat[0].len() {
        let mut r = 0;
        let mut c = 0;
        let mut cross = 0;
        if i > 0 {
          r = buf[i - 1][j];
        }
        if j > 0 {
          c = buf[i][j - 1];
        }
        if i > 0 && j > 0 {
          cross = buf[i - 1][j - 1];
        }
        buf[i][j] = r + c - cross + mat[i][j];
      }
    }
    let mut ret: Vec<Vec<i32>> = vec![vec![0; mat[0].len()]; mat.len()];
    for i in 0..mat.len() {
      for j in 0..mat[0].len() {
        let mut t = 0;

        let mut maxr = 0;
        if i + k as usize >= mat.len() {
          maxr = mat.len() - 1;
        } else {
          maxr = i + k as usize;
        }
        let mut maxc = 0;
        if j + k as usize >= mat[0].len() {
          maxc = mat[0].len() - 1;
        } else {
          maxc = j + k as usize;
        }
        t = buf[maxr][maxc];

        let mut minr: i32 = 0;
        if i - k as usize <= 0 {
          minr = -1;
        } else {
          minr = i as i32 - k - 1;
        }
        let mut minc: i32 = 0;
        if j - k as usize <= 0 {
          minc = -1;
        } else {
          minc = j as i32 - k - 1;
        }
        if minr > -1 {
          t -= buf[minr as usize][maxc];
        }
        if minc > -1 {
          t -= buf[maxr][minc as usize];
        }
        if minr > -1 && minc > -1 {
          t += buf[minr as usize][minc as usize];
        }
        ret[i][j] = t;
      }
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::matrix_block_sum(vec![vec![0; 5]; 5], 5))
}
