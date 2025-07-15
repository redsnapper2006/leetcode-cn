use std::collections::HashMap;
impl Solution {
  pub fn most_frequent_prime(mat: Vec<Vec<i32>>) -> i32 {
    fn is_prime(n: i32) -> bool {
      n == 2 || n > 1 && n % 2 > 0 && (3..=(n as f64).sqrt() as i32).step_by(2).all(|i| n % i > 0)
    }
    fn add_m(m: &mut HashMap<i32, i32>, base: i32) {
      if base <= 10 {
        return;
      }
      if m.contains_key(&base) {
        m.entry(base).and_modify(|x| *x += 1);
      } else if is_prime(base) {
        m.insert(base, 1);
      }
    }
    let mut m: HashMap<i32, i32> = HashMap::new();

    for r in 0..mat.len() {
      for c in 0..mat[0].len() {
        let base = mat[r][c];

        add_m(&mut m, base);
        for cord in vec![
          vec![-1, 0],
          vec![-1, 1],
          vec![0, 1],
          vec![1, 1],
          vec![1, 0],
          vec![1, -1],
          vec![0, -1],
          vec![-1, -1],
        ] {
          let mut b = base;
          let mut nr = r as i32;
          let mut nc = c as i32;
          loop {
            nr += cord[0];
            nc += cord[1];
            if nr < 0 || nr >= mat.len() as i32 || nc < 0 || nc >= mat[0].len() as i32 {
              break;
            }
            b = b * 10 + mat[nr as usize][nc as usize];
            add_m(&mut m, b);
          }
        }
      }
    }

    let mut ans: i32 = -1;
    let mut cnt: i32 = 0;
    for (k, v) in m {
      if cnt < v {
        ans = k;
        cnt = v;
      } else if cnt == v && ans < k {
        ans = k;
      }
    }
    ans
  }
}
