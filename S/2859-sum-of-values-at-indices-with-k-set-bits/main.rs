struct Solution {}

impl Solution {
  pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    if k == 0 {
      return nums[0];
    }

    let mut buf: Vec<i32> = vec![0];
    let mut cnt: i32 = 0;

    let mut sum: i32 = 0;
    (1..nums.len()).for_each(|idx| {
      let mut zero: usize = 0;
      while zero < buf.len() {
        if buf[zero] == 0 {
          cnt += 1;
          buf[zero] = 1;
          break;
        }

        buf[zero] = 0;
        cnt -= 1;
        zero += 1;
      }
      if zero == buf.len() {
        buf.push(1);
        cnt += 1;
      }
      if cnt == k {
        sum += nums[idx];
      }
    });

    sum
  }
}

fn main() {
  println!(
    "{}",
    Solution::sum_indices_with_k_set_bits(vec![5, 10, 1, 5, 2], 1)
  );
  println!(
    "{}",
    Solution::sum_indices_with_k_set_bits(vec![4, 3, 2, 1], 2)
  );
}
