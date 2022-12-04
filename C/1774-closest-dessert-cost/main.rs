struct Solution {}

impl Solution {
  pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
    let mut container: Vec<i32> = vec![0];
    for t in topping_costs {
      let size: usize = container.len();
      for i in 0..size {
        container.push(container[i] + t);
        container.push(container[i] + 2 * t);
      }
    }
    container.sort();
    container.dedup();

    let mut ret: i32 = 1 << 31 - 1;
    for b in &base_costs {
      let mut s: i32 = 0;
      let mut e: i32 = container.len() as i32 - 1;
      while s <= e {
        let m = s + (e - s) / 2;
        if *b + container[m as usize] <= target {
          s = m + 1;
        } else {
          e = m - 1;
        }
      }

      let mut d1: i32 = *b;
      if (s as usize) < container.len() {
        d1 = *b + container[s as usize];
      }
      let mut d2: i32 = *b;
      if s > 0 {
        d2 = *b + container[s as usize - 1];
      }

      let diff1 = d1 - target;
      let diff2 = d2 - target;
      let iret: i32;
      if diff1.abs() >= diff2.abs() {
        iret = d2;
      } else {
        iret = d1;
      }

      if (iret - target).abs() < (ret - target).abs() {
        ret = iret;
      } else if (iret - target).abs() == (ret - target).abs() && iret < ret {
        ret = iret;
      }
    }

    ret
  }
}

fn main() {
  println!("{}", Solution::closest_cost(vec![1, 7], vec![3, 4], 10));
}
