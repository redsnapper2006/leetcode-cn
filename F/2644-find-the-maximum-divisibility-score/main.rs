struct Solution {}

impl Solution {
  pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
    let mut cnt: i32 = -1;
    let mut ret: i32 = 0;
    divisors.iter().for_each(|&div| {
      let cc = nums
        .iter()
        .map(|n| if n % div == 0 { 1 } else { 0 })
        .collect::<Vec<_>>()
        .iter()
        .sum();
      if cc > cnt || cc == cnt && div < ret {
        cnt = cc;
        ret = div;
      }
    });

    ret
  }
}

fn main() {
  println!(
    "{}",
    Solution::max_div_score(vec![1, 2, 3], vec![1, 2, 3, 2])
  );
}
