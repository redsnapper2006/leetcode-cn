struct Solution {}

impl Solution {
  pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    let mut rl: i32 = matrix.len() as i32 - 1;
    let mut cl: i32 = matrix[0].len() as i32;
    let mut direction = 0;
    while true {
      if direction == 0 {
        if cl == 0 {
          break;
        }
        let ce = c + cl;
        while c < ce {
          ret.push(matrix[r as usize][c as usize]);
          c += 1;
        }
        c -= 1;
        direction = 1;
        cl -= 1;
        r += 1;
      }
      if direction == 1 {
        if rl == 0 {
          break;
        }
        let re = r + rl;
        while r < re {
          ret.push(matrix[r as usize][c as usize]);
          r += 1;
        }
        r -= 1;
        direction = 2;
        rl -= 1;
        c -= 1;
      }
      if direction == 2 {
        if cl == 0 {
          break;
        }
        let ce = c - cl;

        while c > ce {
          ret.push(matrix[r as usize][c as usize]);
          c -= 1;
        }
        c += 1;
        direction = 3;
        cl -= 1;
        r -= 1;
      }
      if direction == 3 {
        if rl == 0 {
          break;
        }
        let re = r - rl;
        while r > re {
          ret.push(matrix[r as usize][c as usize]);
          r -= 1;
        }
        r += 1;
        direction = 0;
        rl -= 1;
        c += 1;
      }
    }
    ret
  }
}

fn main() {
  println!("{}", Solution::spiral_order(vec![vec![0; 5]; 3]));
}
