struct Solution {}

impl Solution {
  pub fn clumsy(n: i32) -> i32 {
    let mut stack: Vec<i32> = Vec::new();
    let mut cnt: i32 = 0;
    let mut accu: i32 = 0;
    for i in (1..=n).rev() {
      if cnt == 0 {
        accu = i;
      } else if cnt == 1 {
        accu *= i;
      } else if cnt == 2 {
        accu /= i;
        stack.push(accu);
        accu = 0;
      } else if cnt == 3 {
        stack.push(i);
      } else {
        cnt = 0;
        accu = i;
      }
      cnt += 1;
    }
    if stack.len() == 0 {
      return accu;
    }
    // println!("{:?} {}", stack, accu);
    let mut ret: i32 = stack[0];
    for i in 1..stack.len() {
      if i % 2 == 1 {
        ret += stack[i];
      } else {
        ret -= stack[i];
      }
    }
    ret - accu
  }
}

fn main() {
  println!("{}", Solution::clumsy(4));
  println!("{}", Solution::clumsy(5));
  println!("{}", Solution::clumsy(10));
}
