impl Solution {
  pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![];

    let mut times: usize = 0;
    let mut steps: Vec<Vec<(i32, i32)>> = vec![
      vec![(-1, 1), (0, 1), (1, 1), (1, 0)],
      vec![(1, -1), (1, 0), (1, 1), (0, 1)],
    ];

    let mut r: i32 = 0;
    let mut c: i32 = 0;
    while ans.len() != mat.len() * mat[0].len() {
      ans.push(mat[r as usize][c as usize]);

      let mut idx: i32 = -1;
      for i in 0..steps[times].len() {
        let nr = r + steps[times][i].0;
        let nc = c + steps[times][i].1;
        if nr >= 0 && nr < mat.len() as i32 && nc >= 0 && nc < mat[0].len() as i32 {
          idx = i as i32;
          break;
        }
      }
      if idx == -1 {
        break;
      }
      r += steps[times][idx as usize].0;
      c += steps[times][idx as usize].1;
      if idx != 0 {
        times += 1;
        times %= 2;
      }
    }
    ans
  }
}
