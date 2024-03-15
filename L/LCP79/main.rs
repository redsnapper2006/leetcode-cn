struct Solution {}

use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
  pub fn extract_mantra(matrix: Vec<String>, mantra: String) -> i32 {
    let mut q: VecDeque<(usize, usize, usize)> = VecDeque::new();
    let mbb = mantra.as_bytes().to_vec();

    let bb = matrix
      .iter()
      .map(|row| row.as_bytes().to_vec())
      .collect::<Vec<Vec<u8>>>();

    q.push_back((0, 0, 0));

    let mut visited: HashMap<(usize, usize, usize), i32> = HashMap::new();
    visited.insert((0, 0, 0), 0);

    while q.len() > 0 {
      let (r, c, idx) = q.pop_front().unwrap();

      if mbb[idx] == bb[r][c] {
        let n_idx = idx + 1;
        if n_idx == mbb.len() {
          return visited.get(&(r, c, idx)).unwrap() + 1;
        }
        q.push_back((r, c, n_idx));
        visited.insert((r, c, n_idx), visited.get(&(r, c, idx)).unwrap() + 1);
      } else {
        for (dr, dc) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
          let nr = r as i32 + dr;
          let nc = c as i32 + dc;
          if nr >= 0 && nr < bb.len() as i32 && nc >= 0 && nc < bb[0].len() as i32 {
            let nr = nr as usize;
            let nc = nc as usize;
            if !visited.contains_key(&(nr, nc, idx)) {
              q.push_back((nr, nc, idx));
              visited.insert((nr, nc, idx), visited.get(&(r, c, idx)).unwrap() + 1);
            }
          }
        }
      }
    }

    -1
  }
}

fn main() {
  println!(
    "{}",
    Solution::extract_mantra(
      vec!["sd".to_string(), "ep".to_string()],
      "speed".to_string()
    )
  )
}
