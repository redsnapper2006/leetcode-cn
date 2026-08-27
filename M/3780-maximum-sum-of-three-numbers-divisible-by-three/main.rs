impl Solution {
  pub fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut buf: [[i32; 3]; 3] = [[i32::MIN; 3]; 3];
    let mut cnt: [usize; 3] = [0; 3];
    for &n in &nums {
      let r = n as usize % 3;
      cnt[r] += 1;
      let t = &mut buf[r];
      if n > t[0] {
        t[2] = t[1];
        t[1] = t[0];
        t[0] = n;
      } else if n > t[1] {
        t[2] = t[1];
        t[1] = n;
      } else if n > t[2] {
        t[2] = n;
      }
    }

    let mut ans: i32 = 0;
    for i in 0..3 {
      if cnt[i] >= 3 {
        ans = ans.max(buf[i][0] + buf[i][1] + buf[i][2]);
      }
    }

    if cnt[0] >= 1 && cnt[1] >= 1 && cnt[2] >= 1 {
      ans = ans.max(buf[0][0] + buf[1][0] + buf[2][0]);
    }
    ans
  }
}
