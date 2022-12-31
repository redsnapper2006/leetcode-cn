struct Solution {}

impl Solution {
  pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut sc: Vec<i32> = seats.clone();
    sc.sort();
    let mut stc: Vec<i32> = students.clone();
    stc.sort();
    (0..sc.len()).map(|i| (sc[i] - stc[i]).abs()).sum()
  }
}
