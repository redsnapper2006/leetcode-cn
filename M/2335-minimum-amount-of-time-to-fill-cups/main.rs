struct Solution {}

impl Solution {
  pub fn fill_cups(amount: Vec<i32>) -> i32 {
    let mut a = amount;
    a.sort();
    if a[0] + a[1] <= a[2] {
      return a[2];
    }
    (a[1] + a[0] - a[2] + 1) / 2 + a[2]
  }
}
