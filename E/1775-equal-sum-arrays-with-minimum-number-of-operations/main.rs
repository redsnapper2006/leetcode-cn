struct Solution {}

impl Solution {
  pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let sum1 = nums1.iter().fold(0, |sum, v| sum + v);
    let sum2 = nums2.iter().fold(0, |sum, v| sum + v);
    if sum1 == sum2 {
      return 0;
    }

    let mut w1 = nums1.clone();
    let mut w2 = nums2.clone();
    let mut diff: i32 = sum2 - sum1;
    if sum1 > sum2 {
      w1 = nums2.clone();
      w2 = nums1.clone();
      diff = -diff;
    }
    w1.sort();
    w2.sort();

    let mut idx1 = 0;
    let mut idx2: i32 = w2.len() as i32 - 1;

    let mut steps: i32 = 0;

    while diff > 0 && (idx1 < w1.len() || idx2 >= 0) {
      let mut d1 = -1;
      if idx1 < w1.len() {
        d1 = 6 - w1[idx1];
      }
      let mut d2 = -1;
      if idx2 >= 0 {
        d2 = w2[idx2 as usize] - 1;
      }
      let mut wd = d1;
      if wd < d2 {
        wd = d2;
      }

      steps += 1;
      diff -= wd;
      if wd == d1 {
        idx1 += 1;
      } else {
        idx2 -= 1;
      }
    }

    if diff > 0 {
      -1
    } else {
      steps
    }
  }
}

fn main() {
  println!("{}", Solution::min_operations(vec![1, 2, 3], vec![2, 3, 1]));
}
