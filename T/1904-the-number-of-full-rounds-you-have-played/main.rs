use std::cmp::Ordering;
impl Solution {
  pub fn number_of_rounds(login_time: String, logout_time: String) -> i32 {
    let login = login_time.split(":").collect::<Vec<&str>>();
    let inh = login[0].parse::<i32>().unwrap();
    let inm = login[1].parse::<i32>().unwrap();
    let mut inf: f64 = inh as f64;
    if inm > 0 && inm <= 15 {
      inf += 0.25;
    } else if inm > 15 && inm <= 30 {
      inf += 0.5;
    } else if inm > 30 && inm <= 45 {
      inf += 0.75;
    } else if inm > 45 && inm <= 59 {
      inf += 1 as f64;
    };

    let logout = logout_time.split(":").collect::<Vec<&str>>();
    let ouh = logout[0].parse::<i32>().unwrap();
    let oum = logout[1].parse::<i32>().unwrap();
    let mut outf: f64 = ouh as f64;
    if logout_time.cmp(&login_time) == Ordering::Less {
      outf += 24 as f64;
    }
    if oum >= 15 && oum < 30 {
      outf += 0.25;
    } else if oum >= 30 && oum < 45 {
      outf += 0.5;
    } else if oum >= 45 && oum <= 59 {
      outf += 0.75;
    };

    if outf < inf {
      0
    } else {
      ((outf - inf) * 4 as f64) as i32
    }
  }
}

struct Solution {}

fn main() {
  println!(
    "{}",
    Solution::number_of_rounds("12:13".to_string(), "12:11".to_string())
  );
}
