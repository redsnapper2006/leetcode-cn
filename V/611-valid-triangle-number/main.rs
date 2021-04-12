struct Solution {}

impl Solution {
  pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
      return 0;
    }
    nums.sort();
    // println!("{:?}", nums);
    let mut sum: i32 = 0;
    for i in 0..nums.len() - 2 {
      for j in (i + 1)..nums.len() - 1 {
        let ll = nums[i] + nums[j];
        let mut s: i32 = j as i32 + 1;
        let mut e: i32 = nums.len() as i32 - 1;
        // println!("{} {} {} {}", nums[i], nums[j], s, e);
        while s <= e {
          let m = s + (e - s) / 2;
          if nums[m as usize] >= ll {
            e = m - 1;
          } else {
            s = m + 1;
          }
        }
        sum += s - j as i32 - 1;
        // println!("{} {} ", s, sum);
      }
    }
    sum
  }
}

fn main() {
  println!("{}", Solution::triangle_number(vec![2, 2, 3, 4]));
}
