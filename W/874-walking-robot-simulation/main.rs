struct Solution {}

impl Solution {
  pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
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
