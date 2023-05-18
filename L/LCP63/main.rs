struct Solution {}

impl Solution {
  pub fn ball_game(num: i32, plate: Vec<String>) -> Vec<Vec<i32>> {
    let matrix: Vec<Vec<u8>> = plate
      .iter()
      .map(|p| p.as_bytes().to_vec())
      .collect::<Vec<Vec<u8>>>();

    fn go(rr: i32, cc: i32, dd: i32, num: i32, matrix: &Vec<Vec<u8>>) -> bool {
      let mut s: i32 = 0;
      let mut r = rr;
      let mut c = cc;
      let mut d = dd;
      while s <= num {
        if matrix[r as usize][c as usize] == 'O' as u8 {
          return true;
        }
        if matrix[r as usize][c as usize] == 'E' as u8 {
          if d == 3 {
            d = 4;
          } else if d == 4 {
            d = 1;
          } else if d == 1 {
            d = 2;
          } else {
            d = 3;
          }
        }
        if matrix[r as usize][c as usize] == 'W' as u8 {
          if d == 3 {
            d = 2;
          } else if d == 4 {
            d = 3;
          } else if d == 1 {
            d = 4;
          } else {
            d = 1;
          }
        }
        if d == 1 {
          r -= 1;
        } else if d == 2 {
          c += 1;
        } else if d == 3 {
          r += 1;
        } else {
          c -= 1;
        }

        s += 1;
        if r < 0 || r >= matrix.len() as i32 || c < 0 || c >= matrix[0].len() as i32 {
          return false;
        }
      }

      false
    }

    let mut res: Vec<Vec<i32>> = Vec::new();
    (1..matrix[0].len() - 1).for_each(|c| {
      if matrix[0][c] == '.' as u8 {
        if go(0, c as i32, 3, num, &matrix) {
          res.push(vec![0, c as i32]);
        }
      }
      if matrix[matrix.len() - 1][c] == '.' as u8 {
        if go(matrix.len() as i32 - 1, c as i32, 1, num, &matrix) {
          res.push(vec![matrix.len() as i32 - 1, c as i32]);
        }
      }
    });

    (1..matrix.len() - 1).for_each(|r| {
      if matrix[r][0] == '.' as u8 {
        if go(r as i32, 0, 2, num, &matrix) {
          res.push(vec![r as i32, 0]);
        }
      }
      if matrix[r][matrix[0].len() - 1] == '.' as u8 {
        if go(r as i32, matrix[0].len() as i32 - 1, 4, num, &matrix) {
          res.push(vec![r as i32, matrix[0].len() as i32 - 1]);
        }
      }
    });

    res
  }
}

fn main() {
  // println!(
  //   "{:?}",
  //   Solution::ball_game(
  //     4,
  //     vec!["..E.".to_string(), ".EOW".to_string(), "..W.".to_string()]
  //   )
  // );

  // println!(
  //   "{:?}",
  //   Solution::ball_game(
  //     5,
  //     vec![
  //       ".....".to_string(),
  //       "..E..".to_string(),
  //       ".WO..".to_string(),
  //       ".....".to_string()
  //     ]
  //   )
  // );

  println!(
    "{:?}",
    Solution::ball_game(
      76,
      vec![
        "E......O..".to_string(),
        "E.........".to_string(),
        "W..E...EW.".to_string(),
        "EE.OE.WWWO".to_string(),
        "O.WEOEWWW.".to_string(),
        ".OW..W....".to_string(),
        "W...EW.WE.".to_string(),
        ".E.W...OW.".to_string(),
        "OW...W.EEO".to_string(),
        "......W.W.".to_string()
      ]
    )
  );
}
