struct Solution {}

impl Solution {
  pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    fn dfs(buf: &mut Vec<Vec<char>>, row: usize, col: usize) -> bool {
      if row == 9 {
        return true;
      }
      if col == 9 {
        return dfs(buf, row + 1, 0);
      }
      if buf[row][col] != '.' {
        return dfs(buf, row, col + 1);
      }

      let mut next = 1;
      while next <= 9 {
        let next_char = (next as u8 + '0' as u8) as char;
        let col_valid = (0..9).all(|cc| buf[row][cc] != next_char);
        let row_valid = (0..9).all(|rr| buf[rr][col] != next_char);
        let square_valid = (row / 3 * 3..row / 3 * 3 + 3)
          .all(|rr| (col / 3 * 3..col / 3 * 3 + 3).all(|cc| buf[rr][cc] != next_char));

        if col_valid && row_valid && square_valid {
          buf[row][col] = next_char;
          if dfs(buf, row, col + 1) {
            return true;
          }
          buf[row][col] = '.';
        }
        next += 1;
      }
      false
    }

    _ = dfs(board, 0, 0);
  }
}

fn main() {
  let mut sudoku = vec![
    vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
  ];
  Solution::solve_sudoku(&mut sudoku);
  println!("{:?}", sudoku);
}
