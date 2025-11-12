impl Solution {
  pub fn min_operations(nums: Vec<i32>) -> i32 {
    fn fn_gcd(mut x: i32, mut y: i32) -> i32 {
      while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
      }
      x
    }

    let mut gcd: i32 = 0;
    let mut cnt1: i32 = 0;
    (0..nums.len()).for_each(|idx| {
      gcd = fn_gcd(gcd, nums[idx]);
      if nums[idx] == 1 {
        cnt1 += 1;
      }
    });
    if gcd > 1 {
      return -1;
    }
    if cnt1 > 0 {
      return nums.len() as i32 - cnt1;
    }

    let mut min_size: usize = nums.len();
    (0..nums.len()).for_each(|i| {
      let mut gcd: i32 = nums[i];
      let mut j: usize = i + 1;
      while j < nums.len() {
        gcd = fn_gcd(gcd, nums[j]);
        if gcd == 1 {
          min_size = min_size.min(j - i + 1);
        }
        j += 1;
      }
    });

    (min_size + nums.len() - 2) as _
  }
}

fn main() {
  println!("{}", Solution::min_operations(vec![2, 6, 3, 4]));
  println!("{}", Solution::min_operations(vec![2, 10, 6, 14]));
  println!("{}", Solution::min_operations(vec![6, 10, 15]));
}

struct Solution {}
