struct Solution {}

impl Solution {
  pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut cnt = 0;
    let mut max = 0;
    for i in nums {
      if i == 1 {
        cnt += 1;
      } else {
        if max < cnt {
          max = cnt;
        }
        cnt = 0;
      }
    }
    if max < cnt {
      max = cnt;
    }
    max
  }
}

fn main() {
  println!("{}", Solution::find_max_consecutive_ones(vec![0; 5]));
}
