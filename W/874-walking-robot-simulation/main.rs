use std::cmp::Ordering;
impl Solution {
  pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut direction: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max: i32 = -1;
    commands.iter().for_each(|&command| {
      if command < 0 {
        direction += if command == -1 { 1 } else { 3 };
        direction %= 4;
        return;
      }

      let startx = x;
      let starty = y;
      let mut endx = x;
      let mut endy = y;
      match direction {
        0 => endy += command,
        1 => endx += command,
        2 => endy -= command,
        _ => endx -= command,
      };

      fn new_pos(obs1: i32, base: i32, g1: i32, g2: i32, le1: i32, le2: i32, offset: i32) -> i32 {
        if obs1.cmp(&base) == Ordering::Equal
          && g1.cmp(&g2) == Ordering::Greater
          && le1.cmp(&le2).is_le()
        {
          (if offset < 0 { le1 } else { le2 }) + offset
        } else if offset < 0 {
          le2
        } else {
          le1
        }
      }

      obstacles.iter().for_each(|obs| {
        match direction {
          0 => {
            endy = new_pos(obs[0], startx, obs[1], starty, obs[1], endy, -1);
          }
          1 => {
            endx = new_pos(obs[1], starty, obs[0], startx, obs[0], endx, -1);
          }
          2 => {
            endy = new_pos(obs[0], startx, starty, obs[1], endy, obs[1], 1);
          }
          _ => {
            endx = new_pos(obs[1], starty, startx, obs[0], endx, obs[0], 1);
          }
        };
      });
      x = endx;
      y = endy;
      if x * x + y * y > max {
        max = x * x + y * y
      }
    });
    max
  }

  pub fn robot_sim2(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut direction: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max: i32 = -1;
    commands.iter().for_each(|&command| {
      match command {
        -1 => {
          direction = match direction {
            0 => 1,
            1 => 2,
            2 => 3,
            _ => 0,
          }
        }
        -2 => {
          direction = match direction {
            0 => 3,
            1 => 0,
            2 => 1,
            _ => 2,
          }
        }
        _ => {
          let startx = x;
          let starty = y;
          let mut endx = x;
          let mut endy = y;
          match direction {
            0 => endy += command,
            1 => endx += command,
            2 => endy -= command,
            _ => endx -= command,
          };
          obstacles.iter().for_each(|obstacle| {
            match direction {
              0 => {
                if obstacle[0] == startx && obstacle[1] > starty && obstacle[1] <= endy {
                  endy = obstacle[1] - 1;
                }
              }
              1 => {
                if obstacle[1] == starty && obstacle[0] > startx && obstacle[0] <= endx {
                  endx = obstacle[0] - 1;
                }
              }
              2 => {
                if obstacle[0] == startx && obstacle[1] >= endy && obstacle[1] < starty {
                  endy = obstacle[1] + 1;
                }
              }
              _ => {
                if obstacle[1] == starty && obstacle[0] >= endx && obstacle[0] < startx {
                  endx = obstacle[0] + 1;
                }
              }
            };
          });
          x = endx;
          y = endy;
          if x * x + y * y > max {
            max = x * x + y * y
          }
        }
      };
    });
    max
  }
}

struct Solution {}
