impl Solution {
  pub fn first_unique_even(nums: Vec<i32>) -> i32 {
    let mut buf: [(usize, i32); 101] = [(0, 0); 101];
    for i in 0..nums.len() {
      let n = nums[i] as usize;
      if n % 2 == 0 {
        if buf[n].1 == 0 {
          buf[n] = (i, 1);
        } else {
          buf[n].1 += 1;
        }
      }
    }
    let mut idx: usize = nums.len();
    let mut ans: i32 = -1;
    for i in 0..101 {
      if i % 2 == 0 && buf[i].1 == 1 && idx > buf[i].0 {
        idx = buf[i].0;
        ans = i as i32;
      }
    }
    ans
  }
}
