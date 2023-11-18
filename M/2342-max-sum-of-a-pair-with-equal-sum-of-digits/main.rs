struct Solution {}

use std::collections::HashMap;
impl Solution {
  pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut m: HashMap<i32, [i32; 2]> = HashMap::new();

    nums.iter().for_each(|&num| {
      let mut n = num;
      let mut sum: i32 = 0;
      while n > 0 {
        sum += n % 10;
        n /= 10;
      }
      let arr = m.entry(sum).or_insert([0, 0]);

      if arr[0] < num {
        arr[1] = arr[0];
        arr[0] = num;
      } else if arr[1] < num {
        arr[1] = num;
      }
    });

    let mut max: i32 = -1;
    for (_, arr) in m.iter() {
      if arr[0] > 0 && arr[1] > 0 {
        max = std::cmp::max(max, arr[0] + arr[1]);
      }
    }

    max
  }
}

fn main() {
  println!("{}", Solution::maximum_sum(vec![1, -2, 0, 3]));
}
