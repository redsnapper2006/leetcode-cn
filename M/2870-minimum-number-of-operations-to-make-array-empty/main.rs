impl Solution {
  pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut buf: Vec<i32> = vec![0; 1000001];
    nums.iter().for_each(|&v| {
      buf[v as usize] += 1;
    });

    let mut ans: i32 = 0;
    for i in 0..1000001 {
      if buf[i] == 1 {
        return -1;
      }
      ans += (buf[i] + 2) / 3;
    }

    ans
  }
}
