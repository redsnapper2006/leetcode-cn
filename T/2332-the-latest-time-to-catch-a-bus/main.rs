struct Solution {}

impl Solution {
  pub fn latest_time_catch_the_bus(buses: Vec<i32>, passengers: Vec<i32>, capacity: i32) -> i32 {
    let mut buses = buses;
    let mut passengers = passengers;
    buses.sort_unstable();
    passengers.sort_unstable();

    let mut idx: usize = 0;
    (0..buses.len() - 1).for_each(|i| {
      let ll = match passengers.binary_search(&buses[i]) {
        Ok(ov) => ov + 1,
        Err(ev) => ev,
      };
      if ll - idx > capacity as usize {
        idx += capacity as usize;
      } else {
        idx = ll;
      }
    });

    let mut ll2 = match passengers.binary_search(&buses[buses.len() - 1]) {
      Ok(ov) => ov + 1,
      Err(ev) => ev,
    } as i32
      - 1;

    let mut ans = buses[buses.len() - 1];
    while ll2 >= 0 && (ll2 - idx as i32 + 1 >= capacity || ans == passengers[ll2 as usize]) {
      if ans == passengers[ll2 as usize] {
        ll2 -= 1;
      } else if ll2 - idx as i32 + 1 < capacity {
        break;
      }

      ans -= 1;
    }
    ans
  }
}

fn main() {
  println!(
    "{}",
    Solution::latest_time_catch_the_bus(vec![10, 20], vec![2, 17, 18, 19], 2)
  );
  println!(
    "{}",
    Solution::latest_time_catch_the_bus(vec![10, 20, 30], vec![4, 11, 13, 19, 21, 25, 26], 2)
  );
  println!(
    "{}",
    Solution::latest_time_catch_the_bus(vec![3], vec![2, 3], 2)
  );
  println!(
    "{}",
    Solution::latest_time_catch_the_bus(vec![2], vec![2], 2)
  );
}
