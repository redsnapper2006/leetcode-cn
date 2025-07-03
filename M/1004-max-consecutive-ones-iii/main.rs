impl Solution {
  pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let mut buf: Vec<i32> = vec![];

    let mut ans: i32 = 0;
    for i in 0..nums.len() {
      if nums[i] == 0 {
        buf.push(i as i32);
      }

      let ii = buf.len() as i32;
      ans = ans.max(
        i as i32
          - if ii > k {
            buf[(ii - k - 1) as usize]
          } else {
            -1
          },
      );
    }
    ans
  }
}
