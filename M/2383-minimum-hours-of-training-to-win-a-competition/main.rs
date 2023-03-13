struct Solution {}

impl Solution {
  pub fn min_number_of_hours(
    initial_energy: i32,
    initial_experience: i32,
    energy: Vec<i32>,
    experience: Vec<i32>,
  ) -> i32 {
    let sum_energy: i32 = energy.iter().sum();
    let r_e: i32 = if sum_energy >= initial_energy {
      sum_energy - initial_energy + 1
    } else {
      0
    };

    let mut r_ex: i32 = 0;
    let mut i_ex: i32 = initial_experience;
    for v in experience {
      if v >= i_ex {
        r_ex += v - i_ex + 1;
        i_ex += v - i_ex + 1;
      }
      i_ex += v;
    }

    r_e + r_ex
  }
}

fn main() {
  println!(
    "{}",
    Solution::min_number_of_hours(3, 3, vec![1, 2], vec![1, 2])
  );
}
