struct Solution {}

impl Solution {
  pub fn min_operations_max_profit(
    customers: Vec<i32>,
    boarding_cost: i32,
    running_cost: i32,
  ) -> i32 {
    let mut remain_customer: i32 = 0;
    let mut ret: i32 = 0;
    let mut times: i32 = 0;
    let mut max: i32 = 0;
    let mut ret_times: i32 = -1;

    let mut idx: usize = 0;

    while idx < customers.len() || remain_customer > 0 {
      if idx < customers.len() {
        remain_customer += customers[idx];
        idx += 1;
      }

      let mut board_count: i32 = 4;
      if remain_customer < 4 {
        board_count = remain_customer;
      }

      remain_customer -= board_count;
      ret += board_count * boarding_cost - running_cost;
      times += 1;
      if ret > 0 && ret > max {
        max = ret;
        ret_times = times;
      }
    }

    ret_times
  }
}
