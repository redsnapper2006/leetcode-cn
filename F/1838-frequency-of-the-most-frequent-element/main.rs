struct Solution {}

impl Solution {
  pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
    let mut n : Vec<i32> = nums.clone();
    n.sort();

    let mut ret: usize = 1;
    let mut sum: i32 = 0;
    let mut diff: Vec<i32> = vec![0; n.len()];
    let mut diff_sum: Vec<i32> = vec![0; n.len()];
    let mut end: usize = n.len() - 1;

    for i in (0..n.len() - 1).rev() {
      diff[i] = n[i + 1] - n[i];
      diff_sum[i] = diff_sum[i + 1] + diff[i];
      sum += diff_sum[i] - diff_sum[end];

      if sum <= k && end - i + 1 > ret as usize {
        ret = end - i + 1 ;
      }
      while end >= i && sum > k {
        end -= 1;
        sum -= diff[end] * (end - i + 1) as i32;
      }
      if end - i + 1 > ret {
        ret = end - i + 1;
      }
    }
    ret as i32
  }
}

fn main() {
  println!("{}",Solution::max_frequency(vec![1, 2, 4], 5))
}
