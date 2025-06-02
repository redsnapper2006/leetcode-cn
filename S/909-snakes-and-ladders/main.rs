use std::cmp::Ordering;
use std::collections::HashSet;
// use std::cmp::Reverse;
use std::collections::BinaryHeap;
pub struct G {
  i: i32,
  s: i32,
}
impl PartialOrd for G {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.i == other.i {
      other.s.partial_cmp(&self.s)
    } else {
      other.i.partial_cmp(&self.i)
    }
  }
}
impl PartialEq for G {
  fn eq(&self, other: &Self) -> bool {
    self.i == other.i && self.s == other.s
  }
}
impl Ord for G {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}
impl Eq for G {}

impl Solution {
  pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; board[0].len()]; board.len()];
    dp[board.len() - 1][0] = 0;
    let n = board.len() as i32;

    let mut q: BinaryHeap<G> = BinaryHeap::new();
    let mut visit: HashSet<(i32, i32)> = HashSet::new();
    q.push(G { i: 1, s: 0 });
    visit.insert((1, 0));

    fn row_col(idx: i32, n: usize) -> (usize, usize) {
      // println!("{} {}", idx, n);
      let row = n - 1 - ((idx - 1) as usize / n);
      let col = if ((idx - 1) as usize / n) % 2 == 0 {
        (idx - 1) as usize % n
      } else {
        n - 1 - ((idx - 1) as usize % n)
      };
      (row, col)
    }

    while q.len() > 0 {
      let current = q.pop().unwrap();
      // println!("cur i {} s {}", current.i, current.s);
      if current.i == n * n {
        return current.s;
      }

      // println!("before dp ");
      // for t in &dp {
      //   println!("{:?}", t);
      // }

      let mut offset: i32 = 1;
      while offset <= 6 {
        let next = current.i + offset;
        if next > n * n {
          break;
        }

        let (row, col) = row_col(next, board.len());
        // println!("next {} row {} col {}", next, row, col);

        let ng = G {
          i: next,
          s: current.s + 1,
        };
        if !visit.contains(&(next, current.s + 1))
          && dp[row][col] > current.s + 1
          && board[row][col] == -1
        {
          dp[row][col] = current.s + 1;
          q.push(ng);
          visit.insert((next, current.s + 1));
        }
        if board[row][col] != -1 {
          let nn = board[row][col];
          let (nrow, ncol) = row_col(nn, board.len());
          let nng = G {
            i: nn,
            s: current.s + 1,
          };
          if !visit.contains(&(nn, current.s + 1)) && dp[nrow][ncol] > current.s + 1 {
            dp[nrow][ncol] = current.s + 1;
            q.push(nng);
            visit.insert((nn, current.s + 1));
          }
        }

        offset += 1;
      }
      // println!("after dp ");
      // for t in &dp {
      //   println!("{:?}", t);
      // }
    }

    -1
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::snakes_and_ladders(vec![
      vec![-1, -1, -1, -1, -1, -1],
      vec![-1, -1, -1, -1, -1, -1],
      vec![-1, -1, -1, -1, -1, -1],
      vec![-1, 35, -1, -1, 13, -1],
      vec![-1, -1, -1, -1, -1, -1],
      vec![-1, 15, -1, -1, -1, -1]
    ])
  );

  println!(
    "{}",
    Solution::snakes_and_ladders(vec![
      vec![-1, -1],
      vec![-1, 3]
    ])
  );
  println!(
    "{}",
    Solution::snakes_and_ladders(vec![vec![1, 1, -1], vec![1, 1, 1], vec![-1, 1, 1]])
  );
}
