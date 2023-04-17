struct Solution {}

impl Solution {
  pub fn count_days_together(
    arrive_alice: String,
    leave_alice: String,
    arrive_bob: String,
    leave_bob: String,
  ) -> i32 {
    let mut left = arrive_alice;
    if left < arrive_bob {
      left = arrive_bob;
    }

    let mut right = leave_alice;
    if right > leave_bob {
      right = leave_bob;
    }

    let day: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    match left <= right {
      true => {
        let lv: Vec<&str> = left.split('-').collect();
        let mut lm: i32 = lv[0].parse::<i32>().unwrap_or(0);
        let ld: i32 = lv[1].parse::<i32>().unwrap_or(0);

        let rv: Vec<&str> = right.split('-').collect();
        let rm: i32 = rv[0].parse::<i32>().unwrap_or(0);
        let rd: i32 = rv[1].parse::<i32>().unwrap_or(0);

        let mut sum: i32 = 0;
        while lm < rm {
          sum += day[lm as usize - 1];
          lm += 1;
        }
        sum += rd;
        sum -= ld - 1;
        sum
      }
      false => 0,
    }
  }
}

fn main() {
  println!(
    "{}",
    Solution::count_days_together(
      "08-15".to_string(),
      "08-18".to_string(),
      "08-16".to_string(),
      "08-19".to_string()
    )
  );
}
