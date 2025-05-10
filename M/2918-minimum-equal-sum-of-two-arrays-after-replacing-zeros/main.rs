impl Solution {
  pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
    let sum1 = nums1.iter().map(|x| *x as i64).sum::<i64>();
    let sum2 = nums2.iter().map(|x| *x as i64).sum::<i64>();
    let zero1 = nums1.iter().filter(|&n| *n == 0).count() as i64;
    let zero2 = nums2.iter().filter(|&n| *n == 0).count() as i64;

    if sum1 + zero1 == sum2 + zero2 {
      sum1 + zero1
    } else if sum1 + zero1 > sum2 + zero2 && zero2 > 0 {
      sum1 + zero1
    } else if sum2 + zero2 > sum1 + zero1 && zero1 > 0 {
      sum2 + zero2
    } else {
      -1
    }
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::min_sum(vec![3, 2, 0, 1, 0], vec![6, 5, 0]));

  println!("{}", Solution::min_sum(vec![2, 0, 2, 0], vec![1, 4]));
}
