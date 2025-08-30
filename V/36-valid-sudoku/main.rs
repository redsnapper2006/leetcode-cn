impl Solution {
  pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    fn set_and_compare(buf: &mut Vec<i32>, c: char) -> bool {
      if c == '.' {
        return true;
      }
      let off = (c as u8 - b'0') as usize;
      buf[off - 1] += 1;
      buf[off - 1] == 1
    }
    for r in 0..9 {
      let mut buf: Vec<i32> = vec![0; 9];
      for c in 0..9 {
        if !set_and_compare(&mut buf, board[r][c]) {
          return false;
        }
      }
    }

    for c in 0..9 {
      let mut buf: Vec<i32> = vec![0; 9];
      for r in 0..9 {
        if !set_and_compare(&mut buf, board[r][c]) {
          return false;
        }
      }
    }

    for r in 0..3 {
      for c in 0..3 {
        let mut buf: Vec<i32> = vec![0; 9];
        for ro in 0..3 {
          for co in 0..3 {
            if !set_and_compare(&mut buf, board[r * 3 + ro][c * 3 + co]) {
              return false;
            }
          }
        }
      }
    }
    true
  }
}
