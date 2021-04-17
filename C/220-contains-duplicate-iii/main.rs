struct Solution {}

impl Solution {
  pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
    let mut buf: Vec<i32> = nums.clone();
    buf.truncate(k as usize + 1);
    buf.sort();

    for i in 1..buf.len() {
      let diff: u64 = (buf[i] - buf[i - 1]) as u64;
      if diff <= t as u64 {
        return true;
      }
    }

    for i in (k + 1)..nums.len() as i32 {
      let mut s: i32 = 0;
      let mut e: i32 = buf.len() as i32 - 1;
      let mut idx: i32 = -1;
      while s <= e {
        let m = s + (e - s) / 2;
        if buf[m as usize] > nums[i as usize - k as usize - 1] {
          e = m - 1;
        } else if buf[m as usize] < nums[i as usize - k as usize - 1] {
          s = m + 1
        } else {
          idx = m;
          break;
        }
      }
      buf.remove(idx as usize);

      s = 0;
      e = buf.len() as i32 - 1;
      while s <= e {
        let m = s + (e - s) / 2;
        if buf[m as usize] > nums[i as usize] {
          e = m - 1;
        } else {
          s = m + 1;
        }
      }
      buf.insert(s as usize, nums[i as usize]);

      if s > 0 {
        let diff: u64 = (buf[s as usize] - buf[s as usize - 1]) as u64;
        if diff <= t as u64 {
          return true;
        }
      }
      if (s as usize) < buf.len() - 1 {
        let diff: u64 = (buf[s as usize + 1] - buf[s as usize]) as u64;
        if diff <= t as u64 {
          return true;
        }
      }
    }
    false
  }
}

fn main() {
  println!(
    "{}",
    Solution::contains_nearby_almost_duplicate(vec![9, 2, 3, 4, 5, 4, 3, 2, 1], 3, 2)
  );
}
