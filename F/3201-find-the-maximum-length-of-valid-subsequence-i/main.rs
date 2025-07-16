impl Solution {
  pub fn maximum_length(nums: Vec<i32>) -> i32 {
    // odd, even, odd-even
    let mut buf: Vec<(i32, i32)> = vec![(0, 0); 3];
    if nums[0] % 2 == 0 {
      buf[1] = (1, 0);
      buf[2] = (1, 0);
    } else {
      buf[0] = (1, 0);
      buf[2] = (1, 1);
    }

    for i in 1..nums.len() {
      if nums[i] % 2 == 0 {
        buf[1] = (buf[1] + 1, 0);
      } else {
        buf[0] = (buf[0] + 1, 0);
      }
      match (nums[i] % 2, buf[2].1) {
        (0, 1) => {
          buf[2] = (buf[2].0 + 1, 0);
        }
        (1, 0) => {
          buf[2] = (buf[2].0 + 1, 1);
        }
        (_, _) => {}
      };
    }
    buf[0].1.max(buf[1].1).max(buf[2].1)
  }
}
