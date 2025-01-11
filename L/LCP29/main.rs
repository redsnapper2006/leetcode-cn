struct Solution {}

impl Solution {
  pub fn orchestra_layout(num: i32, x_pos: i32, y_pos: i32) -> i32 {
    let n: u64 = num as u64;
    let x: u64 = x_pos as u64;
    let y: u64 = y_pos as u64;
    if x <= y {
      let mut k: u64 = x;
      if k > n - 1 - y {
        k = n - 1 - y;
      }
      return ((4 * k * (n - k) + (x - k + y - k + 1) - 1) % 9) as i32 + 1;
    }

    let mut k: u64 = y;
    if k > n - 1 - x {
      k = n - 1 - x;
    }
    k += 1;
    ((4 * k * (n - k) - (x - (k - 1) + y - (k - 1) - 1) - 1) % 9) as i32 + 1
  }

  pub fn orchestra_layout_recur(num: i32, x_pos: i32, y_pos: i32) -> i32 {
    let mut buf: Vec<Vec<i32>> = vec![vec![0; num as usize]; num as usize];
    let mut state = 0;
    let mut cnt = 0;
    let mut r: i32 = 0;
    let mut c: i32 = 0;
    while cnt < num * num {
      cnt += 1;

      buf[r as usize][c as usize] = cnt;
      if state == 0 {
        if c + 1 < num && buf[r as usize][c as usize + 1] == 0 {
          c += 1;
        } else {
          state = 1;
          r += 1;
        }
      } else if state == 1 {
        if r + 1 < num && buf[r as usize + 1][c as usize] == 0 {
          r += 1;
        } else {
          state = 2;
          c -= 1;
        }
      } else if state == 2 {
        if c - 1 >= 0 && buf[r as usize][c as usize - 1] == 0 {
          c -= 1;
        } else {
          state = 3;
          r -= 1;
        }
      } else {
        if r - 1 >= 0 && buf[r as usize - 1][c as usize] == 0 {
          r -= 1;
        } else {
          state = 0;
          c += 1;
        }
      }
    }
    // print!("{:?}", buf);
    let mut m = buf[x_pos as usize][y_pos as usize] % 9;
    if m == 0 {
      m = 9;
    }
    m
  }
}

fn main() {
  println!("{}", Solution::orchestra_layout(4, 1, 2));

  println!("{}", Solution::orchestra_layout(5059, 3347, 2169));
}
