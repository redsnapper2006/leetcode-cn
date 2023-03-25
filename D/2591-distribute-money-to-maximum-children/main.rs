struct Solution {}

impl Solution {
  pub fn dist_money(money: i32, children: i32) -> i32 {
    if money < children {
      return -1;
    }
    let mut cnt = (money - children) / 7;
    let m = (money - children) % 7;
    if cnt >= children {
      if m > 0 || cnt > children {
        return children - 1;
      }
      return children;
    }
    if cnt > 0 && cnt == children - 1 && m == 3 {
      cnt -= 1;
    }
    if cnt < 0 {
      return -1;
    }
    cnt
  }
}
