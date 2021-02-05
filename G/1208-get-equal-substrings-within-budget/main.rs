struct Solution {}

impl Solution {
  pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let sb = s.as_bytes();
    let tb = t.as_bytes();

    let mut buf: Vec<i32> = vec![0; sb.len()];
    for i in 0..sb.len() {
      let mut diff: i32 = sb[i] as i32 - tb[i] as i32;
      if diff < 0 {
        diff = -diff;
      }
      buf[i] = diff;
    }

    let mut sum = 0;
    let mut ret: i32 = 0;
    let mut start = 0;
    let mut end = 0;
    while end < buf.len() {
      sum += buf[end];
      while sum > max_cost {
        sum -= buf[start];
        start += 1;
      }
      if end as i32 - start as i32 + 1 > ret {
        ret = end as i32 - start as i32 + 1;
      }
      end += 1;
    }

    ret as i32
  }

  pub fn equal_substringv2(s: String, t: String, max_cost: i32) -> i32 {
    let sb = s.as_bytes();
    let tb = t.as_bytes();

    let mut buf: Vec<i32> = vec![0; sb.len() + 1];
    for i in 0..sb.len() {
      let mut diff: i32 = sb[i] as i32 - tb[i] as i32;
      if diff < 0 {
        diff = -diff;
      }
      buf[i + 1] = diff;
    }

    let mut sum = 0;
    let mut ret: i32 = 0;
    for i in 1..buf.len() {
      sum += buf[i];
      buf[i] = sum;

      let mut s: i32 = 0;
      let mut e: i32 = i as i32 - 1;
      while s <= e {
        let m = s + (e - s) / 2;
        if buf[i] - buf[m as usize] <= max_cost {
          e = m - 1;
        } else {
          s = m + 1;
        }
      }
      if i as i32 - e - 1 > ret {
        ret = i as i32 - e - 1;
      }
    }
    ret as i32
  }
}

fn main() {
  println!(
    "{}",
    Solution::equal_substring(String::from("abcd"), String::from("bcdf"), 3)
  );
}
