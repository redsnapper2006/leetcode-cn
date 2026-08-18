impl Solution {
  pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    if k == nums.len() as i32 {
      return *nums.iter().max().unwrap();
    }
    let mut buf = [0i32; 51];
    for &v in &nums {
      buf[v as usize] += 1;
    }
    match k {
      1 => {
        for i in (0..51).rev() {
          if buf[i] == 1 {
            return i as i32;
          }
        }
        -1
      }
      _ => {
        let mut ans: i32 = -1;
        if buf[nums[0] as usize] == 1 {
          ans = ans.max(nums[0]);
        }
        let last_val = *nums.last().unwrap();
        if buf[last_val as usize] == 1 {
          ans = ans.max(last_val);
        }

        ans
      }
    }
  }
}
