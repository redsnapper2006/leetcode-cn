struct Solution {}

impl Solution {
  pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
    let mut start: usize = 0;
    let mut end: usize = 0;
    let mut buf: Vec<i32> = Vec::new();
    let mut ret = 0;
    buf.push(nums[0]);
    while end < nums.len() {
      if buf[buf.len() - 1] - buf[0] <= limit {
        if buf.len() > ret {
          ret = buf.len();
        }
        end += 1;
        if end >= nums.len() {
          break;
        }
        let mut s: i32 = 0;
        let mut e: i32 = buf.len() as i32 - 1;
        while s <= e {
          let m = s + (e - s) / 2;
          // println!("{}",end);
          if buf[m as usize] > nums[end] {
            e = m - 1;
          } else {
            s = m + 1;
          }
        }
        buf.insert(s as usize, nums[end]);
      } else {
        let mut s = 0;
        let mut e = buf.len() - 1;
        let mut idx = 0;
        while s <= e {
          let m = s + (e - s) / 2;
          if buf[m] > nums[start] {
            e = m - 1;
          } else if buf[m] < nums[start] {
            s = m + 1;
          } else {
            idx = m;
            break;
          }
        }
        buf.remove(idx);
        start += 1;
      }
    }
    ret as i32
  }
}

fn main() {
  println!("{}", Solution::longest_subarray(vec![0; 5], 3));
}
