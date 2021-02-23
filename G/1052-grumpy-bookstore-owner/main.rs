struct Solution {}

impl Solution {
  pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
    let mut mad = vec![0; customers.len()];
    let mut common = vec![0; customers.len()];
    let mut madsum = 0;
    let mut commonsum = 0;
    for i in 0..customers.len() {
      if grumpy[i] == 0 {
        madsum += customers[i];
      }
      commonsum += customers[i];
      mad[i] = madsum;
      common[i] = commonsum;
    }
    // for x in &mad {
    //     println!("mad {}", x);
    // }
    // for x in &common {
    //     println!("common {}", x);
    // }
    // println!("madsum comosum {} {}", madsum, commonsum);
    let mut max = 0;
    for i in (x - 1) as usize..customers.len() {
      // println!("idx {}", i);
      let mut p = 0;
      let mut cmn = 0;
      if i >= x as usize {
        p = mad[i - x as usize];
        cmn = common[i] - common[i - x as usize];
      // println!("x i {} {}", i, x);
      } else {
        cmn = common[i];
      }
      let n = madsum - mad[i];
      // println!("p cmn n {} {} {}", p, cmn, n);
      let cur = p + cmn + n;
      if cur > max {
        max = cur;
      }
    }
    max
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_satisfied(
      vec![1, 0, 1, 2, 1, 1, 7, 5],
      vec![0, 1, 0, 1, 0, 1, 0, 1],
      3
    )
  );
}
