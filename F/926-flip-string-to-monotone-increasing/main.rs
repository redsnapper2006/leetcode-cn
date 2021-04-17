struct Solution {}

impl Solution {
  pub fn min_flips_mono_incr(s: String) -> i32 {
    let bb = s.as_bytes().to_vec();
    let mut idx: usize = 0;
    while bb[idx] == '0' as u8 {
      idx += 1;
    }
    if idx == bb.len() {
      return 0;
    }
    let mut zero_one: Vec<i32> = Vec::new();
    let mut state: u8 = '1' as u8;
    let mut cnt = 0;
    for i in idx..bb.len() {
      if bb[i] == state {
        cnt += 1;
      } else {
        zero_one.push(cnt);
        cnt = 1;
        state = bb[i];
      }
    }
    zero_one.push(cnt);
    if zero_one.len() == 1 {
      return 0;
    }
    let mut zero_cnt: i32 = 0;
    let mut one_cnt: i32 = 0;
    let mut end = zero_one.len();
    if zero_one.len() % 2 == 1 {
      end = zero_one.len() - 1;
    }

    let mut one_sum: Vec<i32> = Vec::new();
    let mut zero_sum: Vec<i32> = Vec::new();
    for i in 0..end {
      if i % 2 == 0 {
        one_cnt += zero_one[i];
        one_sum.push(one_cnt.clone());
      } else {
        zero_cnt += zero_one[i];
        zero_sum.push(zero_cnt.clone());
      }
    }

    let mut ret = one_cnt;
    if ret > zero_cnt {
      ret = zero_cnt;
    }
    for i in 0..one_sum.len() {
      if ret > zero_cnt - zero_sum[i] + one_sum[i] {
        ret = zero_cnt - zero_sum[i] + one_sum[i]
      }
    }

    ret
  }
}

fn main() {
  println!("{}", Solution::min_flips_mono_incr(String::from("00110")));
  println!("{}", Solution::min_flips_mono_incr(String::from("010110")));
  println!(
    "{}",
    Solution::min_flips_mono_incr(String::from("00011000"))
  );
  println!(
    "{}",
    Solution::min_flips_mono_incr(String::from("0011001100"))
  );
  println!(
    "{}",
    Solution::min_flips_mono_incr(String::from("10011111110010111011"))
  );

  println!(
    "{}",
    Solution::min_flips_mono_incr(String::from("101010111001010000011101101110"))
  );
}
