impl Solution {
  pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
    power.sort_unstable();

    let mut i2: (i32, i64) = (0, 0);
    let mut i1: (i32, i64) = (0, 0);
    let mut i0: (i32, i64) = (0, 0);
    for i in 0..power.len() {
      if i > 0 && power[i] == power[i - 1] {
        i0.1 += power[i] as i64;
        continue;
      }
      let mut mx: i64 = i2.1;
      if i1.0 <= power[i] - 3 {
        mx = mx.max(i1.1);
      }
      if i0.0 <= power[i] - 3 {
        mx = mx.max(i0.1);
      }
      i2.0 = i1.0;
      i2.1 = i2.1.max(i1.1);
      i1.0 = i0.0;
      i1.1 = i1.1.max(i0.1);
      i0 = (power[i], mx + power[i] as i64);
    }
    i2.1.max(i1.1).max(i0.1)
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::maximum_total_damage(vec![1, 1, 3, 4]));
  println!("{}", Solution::maximum_total_damage(vec![7, 1, 6, 6]));
  println!(
    "{}",
    Solution::maximum_total_damage(vec![5, 9, 2, 10, 2, 7, 10, 9, 3, 8])
  );
  println!(
    "{}",
    Solution::maximum_total_damage(vec![2, 1, 4, 3, 1, 1, 1, 5])
  );
}
