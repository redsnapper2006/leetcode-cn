impl Solution {
  pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
    let nums: Vec<i64> = nums.iter().map(|x| *x as i64).collect();
    let mut s1mn: i64 = i64::MAX;
    let mut s1sum: i64 = 0;
    let mut s2sum: i64 = 0;
    let mut s3mn: i64 = i64::MAX;
    let mut s3sum: i64 = 0;

    let mut state: i32 = -1;
    let mut ans: i64 = i64::MIN;
    for i in 1..nums.len() {
      if nums[i] == nums[i - 1] {
        s1mn = i64::MAX;
        s1sum = 0;
        s2sum = 0;
        s3mn = i64::MAX;
        s3sum = 0;

        state = -1;
        continue;
      }

      if state == -1 {
        if nums[i] < nums[i - 1] {
          continue;
        }

        s1sum = nums[i] + nums[i - 1];
        s1mn = 0;
        state = 0;
      } else if state == 0 {
        if nums[i] < nums[i - 1] {
          s2sum = nums[i];
          state = 1;
        } else {
          s1mn = s1mn.min(s1sum - nums[i - 1]);
          s1sum += nums[i];
        }
      } else if state == 1 {
        if nums[i] < nums[i - 1] {
          s2sum += nums[i];
        } else {
          s2sum -= nums[i - 1];

          s3sum = nums[i - 1] + nums[i];
          s3mn = 0;
          state = 2;
          ans = ans.max(s1sum - s1mn + s2sum + s3sum);
        }
      } else if state == 2 {
        if nums[i] > nums[i - 1] {
          s3mn = s3mn.min(s3sum - nums[i - 1]);
          s3sum += nums[i];
          ans = ans.max(s1sum - s1mn + s2sum + s3sum);
        } else {
          s1mn = s3mn;
          s1sum = s3sum;

          s2sum = nums[i];
          state = 1;
        }
      }
    }
    ans
  }
}

struct Solution {}

fn main() {
  println!("{}", Solution::max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]));

  println!("{}", Solution::max_sum_trionic(vec![1, 4, 2, 7]));
}
