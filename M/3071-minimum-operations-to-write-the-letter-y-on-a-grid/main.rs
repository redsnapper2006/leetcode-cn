impl Solution {
  pub fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();

    let mut zero_total: i32 = 0;
    let mut one_total: i32 = 0;
    let mut two_total: i32 = 0;
    (0..n).for_each(|row| {
      (0..n).for_each(|col| {
        match grid[row][col] {
          0 => zero_total += 1,
          1 => one_total += 1,
          _ => two_total += 1,
        };
      });
    });
    let mut zero_y: i32 = 0;
    let mut one_y: i32 = 0;
    let mut two_y: i32 = 0;

    (0..n / 2).for_each(|idx| {
      match grid[idx][idx] {
        0 => zero_y += 1,
        1 => one_y += 1,
        _ => two_y += 1,
      };
      match grid[idx][n - 1 - idx] {
        0 => zero_y += 1,
        1 => one_y += 1,
        _ => two_y += 1,
      };
    });
    (n / 2..n).for_each(|idx| {
      match grid[idx][n / 2] {
        0 => zero_y += 1,
        1 => one_y += 1,
        _ => two_y += 1,
      };
    });

    zero_total -= zero_y;
    one_total -= one_y;
    two_total -= two_y;

    let mut ret: i32 = (n * n) as i32;
    if ret > one_y + two_y + zero_total + two_total {
      ret = one_y + two_y + zero_total + two_total;
    }
    if ret > one_y + two_y + zero_total + one_total {
      ret = one_y + two_y + zero_total + one_total;
    }
    if ret > zero_y + two_y + one_total + two_total {
      ret = zero_y + two_y + one_total + two_total;
    }
    if ret > zero_y + two_y + zero_total + one_total {
      ret = zero_y + two_y + zero_total + one_total;
    }
    if ret > zero_y + one_y + one_total + two_total {
      ret = zero_y + one_y + one_total + two_total;
    }
    if ret > zero_y + one_y + zero_total + two_total {
      ret = zero_y + one_y + zero_total + two_total;
    }
    ret
  }
}
