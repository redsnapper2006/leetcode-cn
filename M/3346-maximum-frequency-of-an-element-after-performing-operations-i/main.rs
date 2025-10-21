impl Solution {
  pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
    let k = k as usize;
    let mut buf: Vec<i32> = vec![0; 100001];
    nums.iter().for_each(|&v| {
      buf[v as usize] += 1;
    });
    for i in 1..buf.len() {
      buf[i] += buf[i - 1];
    }

    let mut ans: i32 = 0;
    for i in 1..100001 {
      ans = ans.max(
        buf[i] - buf[i - 1]
          + num_operations.min(
            buf[i - 1] - if i > k { buf[i - k - 1] } else { 0 }
              + if i + k < buf.len() {
                buf[i + k]
              } else {
                buf[buf.len() - 1]
              }
              - buf[i],
          ),
      );
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::max_frequency(vec![1, 4, 5], 1, 2));
  println!("{}", Solution::max_frequency(vec![1, 90], 76, 1));
  println!("{}", Solution::max_frequency(vec![58, 80, 5], 58, 2));
}
