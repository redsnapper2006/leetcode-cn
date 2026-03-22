impl Solution {
  pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
    fn t0(i: usize, j: usize, n: usize) -> (usize, usize) {
      (i, j)
    }
    fn t1(i: usize, j: usize, n: usize) -> (usize, usize) {
      (j, n - 1 - i)
    }
    fn t2(i: usize, j: usize, n: usize) -> (usize, usize) {
      (n - 1 - i, n - 1 - j)
    }
    fn t3(i: usize, j: usize, n: usize) -> (usize, usize) {
      (n - 1 - j, i)
    }

    let transform: Vec<fn(usize, usize, usize) -> (usize, usize)> = vec![t0, t1, t2, t3];
    let n = mat.len();
    for t in transform {
      let mut result: bool = true;
      for r in 0..mat.len() {
        for c in 0..mat[0].len() {
          let (nr, nc) = t(r, c, n);
          if mat[r][c] != target[nr][nc] {
            result = false;
            break;
          }
        }
      }
      if result {
        return true;
      }
    }
    false
  }
}
