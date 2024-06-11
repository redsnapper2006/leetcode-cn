struct Solution {}

impl Solution {
  pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
    let mut board = board;

    fn dfs(board: &mut Vec<Vec<char>>, r: i32, c: i32) {
      board[r as usize][c as usize] = '.';
      let cords: Vec<(i32, i32)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
      for cord in cords {
        let nr = r + cord.0;
        let nc = c + cord.1;
        if nr < 0 || nr >= board.len() as i32 || nc < 0 || nc >= board[0].len() as i32 {
          continue;
        }
        if board[nr as usize][nc as usize] == 'X' {
          dfs(board, nr, nc);
        }
      }
    }

    let mut ans: i32 = 0;
    (0..board.len()).for_each(|row| {
      (0..board[0].len()).for_each(|col| {
        if board[row][col] == '.' {
          return;
        }
        ans += 1;
        dfs(&mut board, row as i32, col as i32);
      });
    });

    ans
  }
}
