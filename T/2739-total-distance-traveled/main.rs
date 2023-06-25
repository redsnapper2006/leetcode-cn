struct Solution {}

impl Solution {
  pub fn distance_traveled(main_tank: i32, additional_tank: i32) -> i32 {
    let mut main: i32 = main_tank;
    let mut additional: i32 = additional_tank;

    let mut distance: i32 = 0;

    while main >= 5 {
      distance += (main/5) * 50;
      let transfer = (main/5).min(additional);
      main = main % 5 + transfer;
      additional -= transfer;
    }

    distance + main * 10
  }
}
